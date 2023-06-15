import { arrayify } from "@ethersproject/bytes";
import { HDNodeWallet, ethers } from "ethers";

export const signEthMessage = async (
  message: string,
  ethSigner: HDNodeWallet
) => {
  const messageHash = ethers.hashMessage(message);

  const messageHashBytes = arrayify(messageHash);

  const full_sig = await ethSigner.signMessage(messageHashBytes);

  const full_sig_bytes = arrayify(full_sig);

  const signature = full_sig_bytes.slice(0, 64);
  const recoveryId = full_sig_bytes[64] - 27;

  const msg_digest = arrayify(messageHash);

  const actual_message = Buffer.concat([
    Buffer.from("\x19Ethereum Signed Message:\n32"),
    msg_digest,
  ]);

  return {
    signature,
    recoveryId,
    actual_message,
    full_sig_bytes,
  };
};
