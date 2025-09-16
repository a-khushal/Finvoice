use crate::error::FinvoiceError;
use crate::state::{Invoice, InvoiceStatus, Listing};
use anchor_lang::prelude::*;

#[event]
pub struct InvoiceListed {
    pub invoice: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub expiry: i64,
}

pub fn list_invoice_fn(ctx: Context<ListInvoice>, price: u64, expiry: i64) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    require!(
        invoice.status == InvoiceStatus::Created as u8,
        FinvoiceError::InvalidStatus
    );
    require!(
        ctx.accounts.issuer.is_signer,
        FinvoiceError::MissingIssuerSignature
    );
    require!(price > 0, FinvoiceError::InsufficientPayment);
    require!(
        expiry > Clock::get()?.unix_timestamp,
        FinvoiceError::InvalidDueDate
    );

    let listing = &mut ctx.accounts.listing;
    listing.invoice = invoice.key();
    listing.price = price;
    listing.expiry = expiry;
    listing.seller = ctx.accounts.issuer.key();

    invoice.status = InvoiceStatus::Listed as u8;

    emit!(InvoiceListed {
        invoice: invoice.key(),
        seller: ctx.accounts.issuer.key(),
        price,
        expiry,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ListInvoice<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"invoice", invoice.issuer.as_ref(), invoice.mint_pubkey.as_ref()],
        bump
    )]
    pub invoice: Account<'info, Invoice>,

    #[account(
        init,
        payer = issuer,
        space = 8 + Listing::SIZE,
        seeds = [b"listing", invoice.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,

    pub system_program: Program<'info, System>,
}
