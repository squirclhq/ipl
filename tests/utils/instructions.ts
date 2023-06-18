import * as anchor from "@project-serum/anchor";
import { SquirclDid } from "../../target/types/squircl_did";
import base58 from "bs58";
import { HDNodeWallet } from "ethers";
import { arrayify } from "@ethersproject/bytes";
import bs58 from "bs58";

export const createDIDEVM = async (
  program: anchor.Program<SquirclDid>,
  didStr: string,
  ethSigner: HDNodeWallet,
  signature: Uint8Array,
  recoveryId: number,
  didAccount: anchor.web3.PublicKey,
  actual_message: Buffer,
  payer: any
) => {
  const sig = await program.methods
    .createDidEvm(didStr, {
      addressBase58: base58.encode(arrayify(ethSigner.address.toLowerCase())),
      sigBase58: base58.encode(signature),
      recoveryId: recoveryId,
    })
    .accounts({
      did: didAccount,
      ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      payer: payer.publicKey,
    })
    .preInstructions([
      anchor.web3.Secp256k1Program.createInstructionWithEthAddress({
        ethAddress: ethSigner.address.toLowerCase().slice(2),
        message: actual_message,
        signature: signature,
        recoveryId: recoveryId,
      }),
    ])
    .rpc();

  return sig;
};

export const createDIDSOL = async (
  program: anchor.Program<SquirclDid>,
  didStr: string,
  keypair: anchor.web3.Keypair,
  signature: Uint8Array,
  messageEncoded: Uint8Array,
  didAccount: anchor.web3.PublicKey,
  payer: any
) => {
  const sig = await program.methods
    .createDidSol(didStr, {
      addressBase58: bs58.encode(keypair.publicKey.toBuffer()),
      sigBase58: bs58.encode(signature),
    })
    .accounts({
      did: didAccount,
      ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      payer: payer.publicKey,
    })
    .preInstructions([
      anchor.web3.Ed25519Program.createInstructionWithPublicKey({
        publicKey: keypair.publicKey.toBytes(),
        message: messageEncoded,
        signature: signature,
      }),
    ])
    .rpc();

  return sig;
};
