use anchor_lang::prelude::*;
use anchor_spl::token::{Mint};
use crate::error::FinvoiceError;
use crate::state::{Invoice, InvoiceStatus};

#[event]
pub struct InvoiceCreated {
    pub invoice: Pubkey,
    pub issuer: Pubkey,
    pub amount: u64,
    pub due_date: i64,
}

pub fn initialize_invoice(
    ctx: Context<InitializeInvoice>,
    payer: Option<Pubkey>,
    invoice_amount: u64,
    currency: u16,
    due_date: i64,
    ipfs_cid: [u8; 46],
) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    require!(invoice_amount > 0, FinvoiceError::InvalidAmount);
    require!(
        due_date > Clock::get()?.unix_timestamp,
        FinvoiceError::InvalidDueDate
    );
    require!(
        ctx.accounts.issuer.is_signer,
        FinvoiceError::MissingIssuerSignature
    );

    invoice.issuer = ctx.accounts.issuer.key();
    invoice.payer = payer;
    invoice.invoice_amount = invoice_amount;
    invoice.currency = currency;
    invoice.due_date = due_date;
    invoice.status = InvoiceStatus::Created as u8;
    invoice.mint_pubkey = ctx.accounts.invoice_mint.key();
    invoice.ipfs_cid = ipfs_cid;
    invoice.attestor_pubkey = Pubkey::default();
    invoice.attestor_sig = [0u8; 64];
    invoice.created_at = Clock::get()?.unix_timestamp;

    emit!(InvoiceCreated {
        invoice: invoice.key(),
        issuer: invoice.issuer,
        amount: invoice.invoice_amount,
        due_date: invoice.due_date,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(payer: Option<Pubkey>, invoice_amount: u64, currency: u16, due_date: i64, ipfs_cid: [u8;46])]
pub struct InitializeInvoice<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    /// CHECK: metadata handled off-chain; mint authority expected to be issuer or PDA
    pub invoice_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = issuer,
        space = 8 + Invoice::SIZE,
        seeds = [b"invoice", issuer.key().as_ref(), invoice_mint.key().as_ref()],
        bump
    )]
    pub invoice: Account<'info, Invoice>,
    /// CHECK: PDA that may hold SOL escrow
    #[account(mut)]
    pub escrow_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
