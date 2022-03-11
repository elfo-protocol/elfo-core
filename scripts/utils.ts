
import {PublicKey} from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
import { ELFO_PROGRAM_ID } from './constants';

const utf8 = anchor.utils.bytes.utf8;
 

export const getProtocolSigner = async (): Promise<PublicKey> => {
    const [protocolSigner] = await PublicKey.findProgramAddress(
        [
            utf8.encode("protocol_signer")
        ],
        ELFO_PROGRAM_ID
    );

    return protocolSigner;
}

export const getProcotolState = async (): Promise<PublicKey> => {
    const [protocolState] = await PublicKey.findProgramAddress(
        [
            utf8.encode("protocol_state")
        ],
        ELFO_PROGRAM_ID
    );
    return protocolState;
}