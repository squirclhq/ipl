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
    .createDid(didStr, {
      eth: {
        ethSig: {
          addressBase58: base58.encode(
            arrayify(ethSigner.address.toLowerCase())
          ),
          sigBase58: base58.encode(signature),
          recoveryId: recoveryId,
        },
      },
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
    .createDid(didStr, {
      sol: {
        solSig: {
          addressBase58: bs58.encode(keypair.publicKey.toBuffer()),
          sigBase58: bs58.encode(signature),
        },
      },
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

export const addAddressEVMwithEVMController = async (
  program: anchor.Program<SquirclDid>,
  didStr: string,
  didAccount: anchor.web3.PublicKey,
  payer: any,
  ethSigner: HDNodeWallet,
  newEthSigner: HDNodeWallet,
  controllerSignature: Uint8Array,
  controllerRecoveryId: number,
  newAddressSignature: Uint8Array,
  newAddressRecoveryId: number,
  controllerActualMessage: Buffer,
  newAddressActualMessage: Buffer
) => {
  const sig = await program.methods
    .addAddres(
      didStr,
      {
        eth: {
          ethSig: {
            addressBase58: base58.encode(
              arrayify(newEthSigner.address.toLowerCase())
            ),
            sigBase58: base58.encode(newAddressSignature),
            recoveryId: newAddressRecoveryId,
          },
        },
      },
      {
        eth: {
          ethSig: {
            addressBase58: base58.encode(
              arrayify(ethSigner.address.toLowerCase())
            ),
            sigBase58: base58.encode(controllerSignature),
            recoveryId: controllerRecoveryId,
          },
        },
      }
    )
    .accounts({
      did: didAccount,
      ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      payer: payer.publicKey,
    })
    .preInstructions([
      anchor.web3.Secp256k1Program.createInstructionWithEthAddress({
        ethAddress: ethSigner.address.toLowerCase().slice(2),
        message: controllerActualMessage,
        signature: controllerSignature,
        recoveryId: controllerRecoveryId,
      }),
      anchor.web3.Secp256k1Program.createInstructionWithEthAddress({
        ethAddress: newEthSigner.address.toLowerCase().slice(2),
        message: newAddressActualMessage,
        signature: newAddressSignature,
        recoveryId: newAddressRecoveryId,
      }),
    ])
    .rpc();

  return sig;
};

export const addAddressEVMwithSOLController = async (
  program: anchor.Program<SquirclDid>,
  didStr: string,
  didAccount: anchor.web3.PublicKey,
  payer: any,
  controllerAddress: anchor.web3.PublicKey,
  controllerSignature: Uint8Array,
  controllerMessageEncoded: Uint8Array,
  newEthSigner: HDNodeWallet,
  newAddressSignature: Uint8Array,
  newAddressRecoveryId: number,
  newAddressActualMessage: Buffer
) => {
  console.log("newEthAddress", newEthSigner.address.toLowerCase());
  console.log("newAddressSignature", newAddressSignature);
  console.log("newAddressRecoveryId", newAddressRecoveryId);
  console.log("newAddressActualMessage", newAddressActualMessage);

  const sig = await program.methods
    .addAddres(
      didStr,
      {
        eth: {
          ethSig: {
            addressBase58: base58.encode(
              arrayify(newEthSigner.address.toLowerCase())
            ),
            sigBase58: base58.encode(newAddressSignature),
            recoveryId: newAddressRecoveryId,
          },
        },
      },
      {
        sol: {
          solSig: {
            addressBase58: bs58.encode(controllerAddress.toBuffer()),
            sigBase58: bs58.encode(controllerSignature),
          },
        },
      }
    )
    .accounts({
      did: didAccount,
      ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      payer: payer.publicKey,
    })
    .preInstructions([
      anchor.web3.Ed25519Program.createInstructionWithPublicKey({
        publicKey: controllerAddress.toBytes(),
        message: controllerMessageEncoded,
        signature: controllerSignature,
      }),
      anchor.web3.Secp256k1Program.createInstructionWithEthAddress({
        ethAddress: newEthSigner.address.toLowerCase().slice(2),
        message: newAddressActualMessage,
        signature: newAddressSignature,
        recoveryId: newAddressRecoveryId,
      }),
    ])
    .rpc();

  return sig;
};

export const addAddressSOLWithEVMController = async (
  program: anchor.Program<SquirclDid>,
  didStr: string,
  didAccount: anchor.web3.PublicKey,
  payer: any,
  ethSigner: HDNodeWallet,
  signature: Uint8Array,
  recoveryId: number,
  actual_message: Buffer,
  newAddress: anchor.web3.PublicKey,
  newSignature: Uint8Array,
  newMessageEncoded: Uint8Array
) => {
  const sig = await program.methods
    .addAddres(
      didStr,
      {
        sol: {
          solSig: {
            addressBase58: bs58.encode(newAddress.toBuffer()),
            sigBase58: bs58.encode(newSignature),
          },
        },
      },
      {
        eth: {
          ethSig: {
            addressBase58: base58.encode(
              arrayify(ethSigner.address.toLowerCase())
            ),
            sigBase58: base58.encode(signature),
            recoveryId: recoveryId,
          },
        },
      }
    )
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
      anchor.web3.Ed25519Program.createInstructionWithPublicKey({
        publicKey: newAddress.toBytes(),
        message: newMessageEncoded,
        signature: newSignature,
      }),
    ])
    .rpc();

  return sig;
};

export const addAddrsesSOLWithSOLController = async (
  program: anchor.Program<SquirclDid>,
  didStr: string,
  didAccount: anchor.web3.PublicKey,
  payer: any,
  controllerAddress: anchor.web3.PublicKey,
  controllerSignature: Uint8Array,
  controllerMessageEncoded: Uint8Array,
  newAddress: anchor.web3.PublicKey,
  newSignature: Uint8Array,
  newMessageEncoded: Uint8Array
) => {
  const sig = await program.methods
    .addAddres(
      didStr,
      {
        sol: {
          solSig: {
            addressBase58: bs58.encode(newAddress.toBuffer()),
            sigBase58: bs58.encode(newSignature),
          },
        },
      },
      {
        sol: {
          solSig: {
            addressBase58: bs58.encode(controllerAddress.toBuffer()),
            sigBase58: bs58.encode(controllerSignature),
          },
        },
      }
    )
    .accounts({
      did: didAccount,
      ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      payer: payer.publicKey,
    })
    .preInstructions([
      anchor.web3.Ed25519Program.createInstructionWithPublicKey({
        publicKey: controllerAddress.toBytes(),
        message: controllerMessageEncoded,
        signature: controllerSignature,
      }),
      anchor.web3.Ed25519Program.createInstructionWithPublicKey({
        publicKey: newAddress.toBytes(),
        message: newMessageEncoded,
        signature: newSignature,
      }),
    ])
    .rpc();

  return sig;
};
