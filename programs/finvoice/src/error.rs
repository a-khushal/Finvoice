use anchor_lang::prelude::*;

#[error_code]
pub enum FinvoiceError {
    #[msg("Invalid status for operation")]
    InvalidStatus,
    #[msg("Insufficient payment amount")]
    InsufficientPayment,
    #[msg("Too early to mark default")]
    TooEarly,
    #[msg("The invoice amount must be greater than zero.")]
    InvalidAmount,
    #[msg("The due date must be in the future.")]
    InvalidDueDate,
    #[msg("Issuer signature is required.")]
    MissingIssuerSignature,
    #[msg("Buyer cannot be the same as the issuer.")]
    InvalidBuyer,
    #[msg("Missing payment token account.")]
    MissingPaymentAccount,
}
