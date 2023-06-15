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
import { expect, should } from "chai";
import { signEthMessage } from "./utils/signatures";

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

  const getDIDAccount = (didStr: string) => {
    const hexString = crypto
      .createHash("sha256")
      .update(didStr, "utf-8")
      .digest("hex");

    let seed = Uint8Array.from(Buffer.from(hexString, "hex"));

    let [didAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [seed],
      program.programId
    );

    return didAccount;
  };

  it("should create a new did document with a ethereum wallet", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

    const ethSigner = ethers.Wallet.createRandom();

    const message = `I am creating a new Squircl DID with the address ${ethSigner.address.toLowerCase()}`;

    const { actual_message, signature, recoveryId, full_sig_bytes } =
      await signEthMessage(message, ethSigner);

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

    console.log("create did evm sig", sig);

    const didAccountData = await program.account.did.fetch(didAccount);

    expect(didAccountData.did).to.equal(didStr);
    expect(didAccountData.ethAddresses.length).to.equal(1);
    expect(didAccountData.ethAddresses[0].address).to.equal(
      ethSigner.address.toLowerCase()
    );
    expect(didAccountData.ethAddresses[0].signature).to.equal(
      hexlify(full_sig_bytes)
    );
    expect(didAccountData.ethAddresses[0].role).to.deep.equal({
      controller: {},
    });
    expect(didAccountData.solAddresses).to.deep.equal([]);
  });

  it("should create a new did document with a solana wallet", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

    const keypair = anchor.web3.Keypair.generate();
    const message = `I am creating a new Squircl DID with the address ${keypair.publicKey.toBase58()}`;

    const messageEncoded = Uint8Array.from(Buffer.from(message));

    const signature = nacl.sign.detached(messageEncoded, keypair.secretKey);

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

    console.log("create did sol sig", sig);

    const didAccountData = await program.account.did.fetch(didAccount);

    expect(didAccountData.did).to.equal(didStr);
    expect(didAccountData.solAddresses.length).to.equal(1);
    expect(didAccountData.solAddresses[0].address).to.equal(
      keypair.publicKey.toBase58()
    );
    expect(didAccountData.solAddresses[0].signature).to.equal(
      bs58.encode(signature)
    );
    expect(didAccountData.solAddresses[0].role).to.deep.equal({
      controller: {},
    });
    expect(didAccountData.ethAddresses).to.deep.equal([]);
  });

  it("should not create a did if signature is invalid, eth", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

    const ethSigner = ethers.Wallet.createRandom();

    const message = "random fake message, will cause invalid signature";

    const { actual_message, signature, recoveryId, full_sig_bytes } =
      await signEthMessage(message, ethSigner);

    try {
      await program.methods
        .createDidEvm(didStr, {
          addressBase58: base58.encode(
            arrayify(ethSigner.address.toLowerCase())
          ),
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
    } catch (e) {
      expect(e.toString()).to.equal(
        "AnchorError occurred. Error Code: InvalidSignature. Error Number: 6001. Error Message: Invalid signature."
      );
    }
  });

  it("should not create a did if signature is invalid, sol", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

    const keypair = anchor.web3.Keypair.generate();

    const message = "random fake message, will cause invalid signature";
    const messageEncoded = Uint8Array.from(Buffer.from(message));

    const signature = nacl.sign.detached(messageEncoded, keypair.secretKey);

    try {
      await program.methods
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
    } catch (e) {
      expect(e.toString()).to.equal(
        "AnchorError occurred. Error Code: InvalidSignature. Error Number: 6001. Error Message: Invalid signature."
      );
    }
  });
});
