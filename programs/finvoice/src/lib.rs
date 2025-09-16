pub mod error;
pub mod instructions;
pub mod state;

use crate::instructions::attest_invoice::__client_accounts_attest_invoice;
use crate::instructions::initialize_invoice::__client_accounts_initialize_invoice;
use crate::instructions::list_invoice::__client_accounts_list_invoice;
use crate::instructions::mark_default::__client_accounts_mark_default;
use crate::instructions::buy_invoice::__client_accounts_buy_invoice;
use crate::instructions::settle_invoice::__client_accounts_settle_invoice;

use anchor_lang::prelude::*;
use instructions::attest_invoice::{attest_invoice_fn, AttestInvoice};
use instructions::initialize_invoice::{initialize_invoice_fn, InitializeInvoice};
use instructions::list_invoice::{list_invoice_fn, ListInvoice};
use instructions::mark_default::{mark_default_fn, MarkDefault};
use instructions::buy_invoice::{buy_invoice_fn, BuyInvoice};
use instructions::settle_invoice::{settle_invoice_fn, SettleInvoice};

declare_id!("58mxSPUZ18AGWQqM7oYgYx4nhHW3noknPpCQjdTNykDp");

#[program]
pub mod finvoice {
    use crate::instructions::{buy_invoice::BuyInvoice, settle_invoice::SettleInvoice};

    use super::*;

    pub fn initialize_invoice(
        ctx: Context<InitializeInvoice>,
        payer: Option<Pubkey>,
        invoice_amount: u64,
        currency: u16,
        due_date: i64,
        ipfs_cid: [u8; 46],
    ) -> Result<()> {
        initialize_invoice_fn(ctx, payer, invoice_amount, currency, due_date, ipfs_cid)
    }

    pub fn attest_invoice(ctx: Context<AttestInvoice>, attestor_sig: [u8; 64]) -> Result<()> {
        attest_invoice_fn(ctx, attestor_sig)
    }

    pub fn list_invoice(ctx: Context<ListInvoice>, price: u64, expiry: i64, bump: u8) -> Result<()> {
        list_invoice_fn(ctx, price, expiry, bump)
    }

    pub fn mark_default(ctx: Context<MarkDefault>) -> Result<()> {
        mark_default_fn(ctx)
    }

    pub fn buy_invoice(ctx: Context<BuyInvoice>, amount: u64) -> Result<()> {
        buy_invoice_fn(ctx, amount)
    }

    pub fn settle_invoice(ctx: Context<SettleInvoice>) -> Result<()> {
        settle_invoice_fn(ctx)
    }
}
