
import { getProcotolState, getProtocolSigner } from './utils';

const showProtocolDetails = async () => {
    const protocolSigner = await getProtocolSigner();
    console.log("Protocol Signer: ", protocolSigner.toBase58());

    const protocolState = await getProcotolState();
    console.log("Protocol State: ", protocolState.toBase58());
}

showProtocolDetails().then(() => console.log("Done")).catch(e => console.error(e));
