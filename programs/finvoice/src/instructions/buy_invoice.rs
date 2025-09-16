use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer as SplTransfer};
use crate::error::FinvoiceError;
use crate::state::{Invoice, InvoiceStatus, Listing};

#[derive(Accounts)]
pub struct BuyInvoice<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub issuer: Signer<'info>,
    #[account(mut, seeds = [b"invoice", invoice.issuer.as_ref(), invoice.mint_pubkey.as_ref()], bump)]
    pub invoice: Account<'info, Invoice>,
    #[account(mut, seeds = [b"listing", invoice.key().as_ref()], bump)]
    pub listing: Account<'info, Listing>,

    /// CHECK: PDA for SOL escrow
    #[account(mut, seeds = [b"escrow", invoice.key().as_ref()], bump)]
    pub escrow_vault: UncheckedAccount<'info>,

    // NFT transfer
    pub invoice_mint: Account<'info, Mint>,
    #[account(mut)]
    pub seller_nft_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_nft_ata: Account<'info, TokenAccount>,

    // Optional SPL payment path
    pub payment_mint: Option<Account<'info, Mint>>,
    #[account(mut)]
    pub buyer_token_account: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub escrow_token_account: Option<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_invoice_fn(ctx: Context<BuyInvoice>, amount: u64) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;
    let listing = &ctx.accounts.listing;

    require!(
        invoice.status == InvoiceStatus::Listed as u8,
        FinvoiceError::InvalidStatus
    );
    require!(
        amount >= listing.price,
        FinvoiceError::InsufficientPayment
    );
    require!(ctx.accounts.buyer.key() != invoice.issuer, FinvoiceError::InvalidBuyer);

    if ctx.accounts.payment_mint.is_some() {
        let buyer_token_account = ctx.accounts.buyer_token_account.as_ref().ok_or(FinvoiceError::MissingPaymentAccount)?;
        let escrow_token_account = ctx.accounts.escrow_token_account.as_ref().ok_or(FinvoiceError::MissingPaymentAccount)?;

        let cpi_accounts = SplTransfer {
            from: buyer_token_account.to_account_info(),
            to: escrow_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
    } else {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.escrow_vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.escrow_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    let signer_seeds: &[&[&[u8]]] = &[&[b"invoice", invoice.issuer.as_ref(), invoice.mint_pubkey.as_ref(), &[listing.bump]]];

    let cpi_accounts_nft = SplTransfer {
        from: ctx.accounts.seller_nft_ata.to_account_info(),
        to: ctx.accounts.buyer_nft_ata.to_account_info(),
        authority: ctx.accounts.issuer.to_account_info(),
    };
    let cpi_ctx_nft = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts_nft, signer_seeds);
    anchor_spl::token::transfer(cpi_ctx_nft, 1)?;

    invoice.status = InvoiceStatus::Funded as u8;

    emit!(InvoiceFunded {
        invoice: invoice.key(),
        buyer: ctx.accounts.buyer.key(),
        amount
    });

    Ok(())
}

#[event]
pub struct InvoiceFunded {
    pub invoice: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
}
