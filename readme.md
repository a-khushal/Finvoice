Finvoice — Invoice-as-NFT on Solana

Tokenize invoices as NFTs on Solana so businesses can unlock working capital and investors can buy/trade invoice assets.

Features (MVP)
- Invoice minting with IPFS metadata (NFT or SFT)
- Attestation capture (attestor pubkey + signature)
- Listing & discovery (simple marketplace)
- Buy flow (SOL or SPL payment escrow) + NFT transfer
- Settlement to current NFT holder; default marking after due date

Architecture (high level)
- On-chain: Anchor program (Rust)
- Tokens: SPL + Metaplex Token Metadata for invoice NFT
- Storage: IPFS for invoice docs/metadata
- Optional: Off-chain attestation service + reconciler worker

Program interface (Anchor)
- initialize_invoice(ctx, payer, invoice_amount, currency, due_date, ipfs_cid): Creates invoice PDA, stores metadata, mints NFT.
- attest_invoice(ctx, attestor_sig): Records attestor pubkey + signature on the invoice.
- list_invoice(ctx, price, expiry): Creates listing PDA; sets status = Listed.
- buy_invoice(ctx, amount): Escrows payment (SOL/SPL) and transfers NFT to buyer; sets status = Funded.
- settle_invoice(ctx): Releases escrow to current NFT holder; sets status = Settled.
- mark_default(ctx): After due date, marks invoice as Defaulted.

Accounts / PDAs
- Invoice PDA: ["invoice", issuer, mint] → issuer, payer, amount, currency, due_date, status, mint_pubkey, ipfs_cid, attestor_pubkey/sig, created_at
- Listing PDA: ["listing", invoice] → price, expiry, seller
- Escrow PDA: ["escrow", invoice] → SOL/SPL escrow vault

Quickstart
Prereqs: Rust toolchain, Solana CLI, Anchor CLI.

```bash
# Clone
git clone <repo>
cd finvoice

# Build program
anchor build

# Test
anchor test

# Localnet (in another shell)
solana-test-validator
anchor deploy
```

Repository layout
- programs/finvoice/ — Anchor program
  - src/instructions/ — instruction handlers + accounts
  - src/state.rs — on-chain structs/enums
  - src/error.rs — program errors
- tests/ — Anchor tests (to be expanded)

Safety notes
- Some accounts use UncheckedAccount<'info> for PDAs or recipients. Each such field is documented with a "/// CHECK:" comment and guarded at runtime (PDA seeds, invoke_signed, or transfer-only usage). Prefer typed accounts (SystemAccount, Account<TokenAccount>, Account<Mint>) when possible.

License
MIT or Apache-2.0 (choose one and update this section).


