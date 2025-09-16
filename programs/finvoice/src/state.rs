use anchor_lang::prelude::*;

#[account]
pub struct Invoice {
    pub issuer: Pubkey,
    pub payer: Option<Pubkey>,
    pub invoice_amount: u64,
    pub currency: u16,
    pub due_date: i64,
    pub status: u8, // 0=Created,1=Listed,2=Funded,3=Settled,4=Defaulted
    pub mint_pubkey: Pubkey,
    pub ipfs_cid: [u8; 46],
    pub attestor_pubkey: Pubkey,
    pub attestor_sig: [u8; 64],
    pub created_at: i64,
}

impl Invoice {
    pub const SIZE: usize = 32 + 1 + 32 + 8 + 2 + 8 + 1 + 32 + 46 + 32 + 64 + 8;
}

#[account]
pub struct Listing {
    pub invoice: Pubkey,
    pub price: u64,
    pub expiry: i64,
    pub seller: Pubkey,
}

impl Listing {
    pub const SIZE: usize = 32 + 8 + 8 + 32;
}

#[repr(u8)]
pub enum InvoiceStatus {
    Created = 0,
    Listed = 1,
    Funded = 2,
    Settled = 3,
    Defaulted = 4,
}
