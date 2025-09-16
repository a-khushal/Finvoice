use anchor_lang::prelude::*;
use crate::error::FinvoiceError;
use crate::state::{Invoice, InvoiceStatus};

pub fn mark_default(ctx: Context<MarkDefault>) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    require!(
        Clock::get()?.unix_timestamp > invoice.due_date,
        FinvoiceError::TooEarly
    );

    invoice.status = InvoiceStatus::Defaulted as u8;

    Ok(())
}

#[derive(Accounts)]
pub struct MarkDefault<'info> {
    #[account(
        mut,
        seeds = [b"invoice", invoice.issuer.as_ref(), invoice.mint_pubkey.as_ref()],
        bump,
    )]
    pub invoice: Account<'info, Invoice>,
}