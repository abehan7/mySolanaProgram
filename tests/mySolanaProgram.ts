import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MySolanaProgram } from "../target/types/my_solana_program";

describe("mySolanaProgram", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MySolanaProgram as Program<MySolanaProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
