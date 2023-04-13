import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaProgram } from "../target/types/solana_program";

(async () => {
  const idl = require("../target/idl/solana_program.json");
  const connection = new anchor.web3.Connection(
    anchor.web3.clusterApiUrl("devnet")
  );
  const authority = new anchor.web3.Keypair();
  const wallet = new anchor.Wallet(authority);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "processed",
  });

  const Program_ID = new anchor.web3.PublicKey(idl.metadata.address);
  const program = new anchor.Program(
    idl,
    Program_ID,
    provider
  ) as Program<SolanaProgram>;
  return program;
})();
