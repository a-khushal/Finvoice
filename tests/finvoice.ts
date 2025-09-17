import * as anchor from "@coral-xyz/anchor";
import { Program, BN, web3 } from "@coral-xyz/anchor";
import { Finvoice } from "../target/types/finvoice";
import { createMint } from "@solana/spl-token";
import { expect } from "chai";

describe("finvoice", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.finvoice as Program<Finvoice>;

  it("initializes invoice", async () => {
    const issuer = provider.wallet.publicKey;

    const invoiceMint = await createMint(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      issuer,
      null,
      0
    );

    const [invoicePda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("invoice"), issuer.toBuffer(), invoiceMint.toBuffer()],
      program.programId
    );

    const payer = web3.PublicKey.default;
    const invoiceAmount = new BN(1_000);
    const currency = 1;
    const dueDate = new BN(Math.floor(Date.now() / 1000) + 3600);
    const ipfsCid = Array(46).fill(0);

    const txSig = await program.methods
      .initializeInvoice(payer, invoiceAmount, currency, dueDate, ipfsCid)
      .accounts({
        issuer,
        invoiceMint
      })
      .rpc();

    const invoice = await program.account.invoice.fetch(invoicePda);

    expect(invoice.issuer.toBase58()).to.equal(issuer.toBase58());
    expect(new BN(invoice.invoiceAmount).toString()).to.equal(invoiceAmount.toString());
    expect(invoice.status).to.equal(0);

    console.log("initialize_invoice tx:", txSig);
  });

});
