pub mod error;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;

declare_id!("58mxSPUZ18AGWQqM7oYgYx4nhHW3noknPpCQjdTNykDp");

#[program]
pub mod finvoice {
    use super::*;
    use crate::instructions::{
        initialize_invoice::{self, InitializeInvoice},
        attest_invoice::{self, AttestInvoice},
        list_invoice::{self, ListInvoice},
        mark_default::{self, MarkDefault},
    };

    pub fn initialize_invoice(
        ctx: Context<InitializeInvoice>,
        payer: Option<Pubkey>,
        invoice_amount: u64,
        currency: u16,
        due_date: i64,
        ipfs_cid: [u8; 46],
    ) -> Result<()> {
        initialize_invoice::initialize_invoice(
            ctx,
            payer,
            invoice_amount,
            currency,
            due_date,
            ipfs_cid,
        )
    }

    pub fn attest_invoice(
        ctx: Context<AttestInvoice>,
        attestor_sig: [u8; 64]
    ) -> Result<()> {
        attest_invoice::attest_invoice(ctx, attestor_sig)
    }

    pub fn list_invoice(
        ctx: Context<ListInvoice>, 
        price: u64, 
        expiry: i64
    ) -> Result<()> {
        list_invoice::list_invoice(ctx, price, expiry)
    }

    pub fn mark_default(
        ctx: Context<MarkDefault>
    ) -> Result<()> {
        mark_default::mark_default(ctx)
    }
}


