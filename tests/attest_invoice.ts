import * as anchor from "@coral-xyz/anchor";
import { Program, BN, web3 } from "@coral-xyz/anchor";
import { Finvoice } from "../target/types/finvoice";
import { createMint } from "@solana/spl-token";
import { expect } from "chai";

describe("attest_invoice", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const provider = anchor.getProvider() as anchor.AnchorProvider;
    const program = anchor.workspace.finvoice as Program<Finvoice>;

    let listener: number;

    before(async () => {
        listener = program.addEventListener("invoiceAttested", (event, slot) => {
            console.log("InvoiceAttested event:", event, "at slot", slot);
        });
    });

    after(async () => {
        await program.removeEventListener(listener);
    });

    it("attests invoice", async () => {
        const issuer = provider.wallet.publicKey;
        const invoiceMint = await createMint(
            provider.connection,
            (provider.wallet as anchor.Wallet).payer,
            issuer,
            null,
            0
        );

        const payer = issuer;
        const amount = new BN(1);
        const currency = 0;
        const dueDate = new BN(Math.floor(Date.now() / 1000) + 3600);
        const ipfsCid = new Array(46).fill(0);

        const [invoicePda] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("invoice"), issuer.toBuffer(), invoiceMint.toBuffer()],
            program.programId
        );

        await program.methods
            .initializeInvoice(payer, amount, currency, dueDate, ipfsCid)
            .accounts({
                issuer,
                invoiceMint,
            })
            .rpc();

        const attestor = web3.Keypair.generate();
        const airdropSig = await provider.connection.requestAirdrop(
            attestor.publicKey,
            web3.LAMPORTS_PER_SOL
        );
        const latest = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction(
            { signature: airdropSig, ...latest },
            "confirmed"
        );

        const attestorSig = Array.from(new Uint8Array(64).fill(1));

        await program.methods
            .attestInvoice(attestorSig)
            .accounts({
                attestor: attestor.publicKey,
                invoiceMint: invoicePda,
            })
            .signers([attestor])
            .rpc();

        await new Promise((resolve) => setTimeout(resolve, 1000));

        const invoice = await program.account.invoice.fetch(invoicePda);
        expect(invoice.attestorPubkey.toBase58()).to.equal(attestor.publicKey.toBase58());
        expect(Array.from(invoice.attestorSig)).to.deep.equal(attestorSig);

        console.log(listener);
    });
});