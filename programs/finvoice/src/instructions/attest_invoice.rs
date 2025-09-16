use anchor_lang::prelude::*;
use crate::error::FinvoiceError;
use crate::state::{Invoice, InvoiceStatus};

#[event]
pub struct InvoiceAttested {
    pub invoice: Pubkey,
    pub attestor: Pubkey,
}

pub fn attest_invoice(ctx: Context<AttestInvoice>, attestor_sig: [u8; 64]) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    require!(
        invoice.status == InvoiceStatus::Created as u8
            || invoice.status == InvoiceStatus::Listed as u8,
        FinvoiceError::InvalidStatus
    );
    require!(
        ctx.accounts.attestor.is_signer,
        FinvoiceError::MissingIssuerSignature
    );

    invoice.attestor_pubkey = ctx.accounts.attestor.key();
    invoice.attestor_sig = attestor_sig;

    emit!(InvoiceAttested {
        invoice: invoice.key(),
        attestor: invoice.attestor_pubkey,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct AttestInvoice<'info> {
    pub attestor: Signer<'info>,
    #[account(mut, seeds = [b"invoice", invoice.issuer.as_ref(), invoice.mint_pubkey.as_ref()], bump)]
    pub invoice: Account<'info, Invoice>,
}