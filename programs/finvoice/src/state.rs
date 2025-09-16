use anchor_lang::prelude::*;

#[account]
pub struct Invoice {
    pub issuer: Pubkey,          // 32
    pub payer: Option<Pubkey>,   // 1 (discriminant) + 32 (Pubkey) = 33
    pub invoice_amount: u64,     // 8
    pub currency: u16,           // 2
    pub due_date: i64,           // 8
    pub status: u8,              // 1
    pub mint_pubkey: Pubkey,     // 32
    pub ipfs_cid: [u8; 46],      // 46
    pub attestor_pubkey: Pubkey, // 32
    pub attestor_sig: [u8; 64],  // 64
    pub created_at: i64,         // 8
}

impl Invoice {
    pub const SIZE: usize = 32 + 33 + 8 + 2 + 8 + 1 + 32 + 46 + 32 + 64 + 8;
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
