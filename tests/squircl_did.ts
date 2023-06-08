import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SquirclDid } from "../target/types/squircl_did";
import lumina from "@lumina-dev/test";
import crypto from "crypto";
import nacl from "tweetnacl";
import bs58 from "bs58";
import { Transaction, Wallet, ethers } from "ethers";
import { hexlify, arrayify } from "@ethersproject/bytes";
import base58 from "bs58";

const generateRandomDID = () => {
  const randomBytes = crypto.randomBytes(24); // 24 bytes = 48 characters
  const randomDID = randomBytes.toString("hex");
  return randomDID;
};

lumina();

describe("squircl_identity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SquirclDid as Program<SquirclDid>;

  const payer = anchor.workspace.SquirclIdentity.provider.wallet;

  it("should create a new did document with a ethereum wallet", async () => {
    const eth_signer = ethers.Wallet.createRandom();

    const message = `I am creating a new Squircl DID with the address ${eth_signer.address.toLowerCase()}`;

    console.log(message);

    const messageHash = ethers.hashMessage(message);

    const messageHashBytes = arrayify(messageHash);

    const full_sig = await eth_signer.signMessage(messageHashBytes);

    const full_sig_bytes = arrayify(full_sig);

    const signature = full_sig_bytes.slice(0, 64);
    const recoveryId = full_sig_bytes[64] - 27;

    const eth_address = ethers.computeAddress(eth_signer.publicKey).slice(2);

    const msg_digest = arrayify(messageHash);

    const actual_message = Buffer.concat([
      Buffer.from("\x19Ethereum Signed Message:\n32"),
      msg_digest,
    ]);

    console.log("actual_message", actual_message);
    console.log("digest", msg_digest);

    const sig = await program.methods
      .createDidEvm(
        base58.encode(arrayify("0x" + eth_address)),
        base58.encode(signature),
        base58.encode(actual_message),
        recoveryId
      )
      .accounts({
        // did: didAccount,
        ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        payer: payer.publicKey,
      })
      .preInstructions([
        anchor.web3.Secp256k1Program.createInstructionWithEthAddress({
          ethAddress: eth_address,
          message: actual_message,
          signature: signature,
          recoveryId: recoveryId,
        }),
      ])
      .rpc();

    console.log(sig);
  });

  it("should create a new did document with a solana wallet", async () => {
    const keypair = anchor.web3.Keypair.generate();
    const message = `I am creating a new Squircl DID with the address ${keypair.publicKey.toBase58()}`;

    console.log("message", message);

    const messageEncoded = Uint8Array.from(Buffer.from(message));

    const signature = nacl.sign.detached(messageEncoded, keypair.secretKey);

    const sig = await program.methods
      .createDidSol(
        bs58.encode(keypair.publicKey.toBuffer()),
        bs58.encode(signature),
        bs58.encode(messageEncoded)
      )
      .accounts({
        // did: didAccount,
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

    console.log(sig);
  });
});
