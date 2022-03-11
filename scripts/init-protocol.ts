import { Program, Provider, Wallet } from "@project-serum/anchor";
import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import * as fs from 'fs-extra';
import {homedir} from 'os';
import {join} from 'path';
import idl from "../target/idl/elfo_protocol.json";
import { ElfoProtocol } from "../target/types/elfo_protocol";
import { ELFO_PROGRAM_ID, ENDPOINT } from "./constants";
import { getProcotolState, getProtocolSigner } from "./utils";

const init = async () => {
    // init protocol
  const provider = await getProvider();
  const program = await getProgram(provider);

  const protocolSigner = await getProtocolSigner();
  const protocolState = await getProcotolState();

  await program.rpc.initialize({
    accounts: {
      authority: provider.wallet.publicKey,
      protocolSigner,
      protocolState,
      rent: SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId
    }
  });

  console.log("Elfo protocol Initialized.");
  console.log("ProtocolSigner: ", protocolSigner.toBase58());
  console.log("ProtocolState: ", protocolState.toBase58());

}

const getProgram = async (provider: Provider): Promise<Program<ElfoProtocol>> => {
  return new Program<ElfoProtocol>(idl as ElfoProtocol, ELFO_PROGRAM_ID, provider);
}

const getProvider = async (): Promise<Provider> => {
  return new Provider(
    new Connection(ENDPOINT),
    new Wallet(await getKeyPair()),
    Provider.defaultOptions(),
  );
}

const getKeyPair = async (): Promise<Keypair> => {
  const id = await fs.readJSON(join(homedir(), '.config/solana/id.json'));
  const bytes = Uint8Array.from(id);
  return Keypair.fromSecretKey(bytes);
}

init().then(() => console.log("Done")).catch(e => {
  console.log(e);
})