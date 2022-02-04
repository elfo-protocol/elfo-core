import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SubrinaProtocol } from "../target/types/subrina_protocol";

describe("subrina-protocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SubrinaProtocol as Program<SubrinaProtocol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
