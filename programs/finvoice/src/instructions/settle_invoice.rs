use crate::error::FinvoiceError;
use crate::state::{Invoice, InvoiceStatus};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer as SplTransfer};

#[derive(Accounts)]
pub struct SettleInvoice<'info> {
    #[account(mut, seeds = [b"invoice", invoice.issuer.as_ref(), invoice.mint_pubkey.as_ref()], bump)]
    pub invoice: Account<'info, Invoice>,

    /// CHECK: PDA for SOL escrow
    #[account(mut, seeds = [b"escrow", invoice.key().as_ref()], bump)]
    pub escrow_vault: UncheckedAccount<'info>,

    // Optional SPL path
    pub payment_mint: Option<Account<'info, Mint>>,
    #[account(mut)]
    pub escrow_token_account: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub recipient_token_account: Option<Account<'info, TokenAccount>>,

    /// CHECK: recipient for SOL path
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn settle_invoice_fn(ctx: Context<SettleInvoice>) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;
    require!(
        invoice.status == InvoiceStatus::Funded as u8,
        FinvoiceError::InvalidStatus
    );

    let escrow_bump = ctx.bumps.escrow_vault;
    let invoice_key = invoice.key();
    let signer_seeds: &[&[&[u8]]] = &[&[b"escrow", invoice_key.as_ref(), &[escrow_bump]]];

    if ctx.accounts.payment_mint.is_some() {
        let escrow_token_account = ctx.accounts.escrow_token_account.as_ref().ok_or(FinvoiceError::MissingPaymentAccount)?;
        let recipient_token_account = ctx.accounts.recipient_token_account.as_ref().ok_or(FinvoiceError::MissingPaymentAccount)?;

        let cpi_accounts = SplTransfer {
            from: escrow_token_account.to_account_info(),
            to: recipient_token_account.to_account_info(),
            authority: ctx.accounts.escrow_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        let amount = escrow_token_account.amount;
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        emit!(InvoiceSettled {
            invoice: invoice.key(),
            recipient: ctx.accounts.recipient.key(),
            amount
        });
    } else {
        let amount = **ctx.accounts.escrow_vault.lamports.borrow();
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.escrow_vault.key(),
            &ctx.accounts.recipient.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.escrow_vault.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer_seeds,
        )?;

        emit!(InvoiceSettled {
            invoice: invoice.key(),
            recipient: ctx.accounts.recipient.key(),
            amount
        });
    }

    invoice.status = InvoiceStatus::Settled as u8;
    Ok(())
}

#[event]
pub struct InvoiceSettled {
    pub invoice: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
}
