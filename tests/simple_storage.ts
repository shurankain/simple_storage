import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleStorage } from "../target/types/simple_storage";
import { assert } from "chai";

describe("simple_storage", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.SimpleStorage as Program<SimpleStorage>;
  const storageAccount = anchor.web3.Keypair.generate();

  it("Initializes the storage", async () => {
    await program.methods.initialize().accounts({
      storage: storageAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any).signers([storageAccount]).rpc();

    const account = await program.account.storage.fetch(storageAccount.publicKey);
    assert.equal(account.data, "Hello, Solana!");
  });

  it("Updates the storage", async () => {
    await program.methods.update("New Data").accounts({
      storage: storageAccount.publicKey,
    }).rpc();

    const account = await program.account.storage.fetch(storageAccount.publicKey);
    assert.equal(account.data, "New Data");
  });
});
