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

    const eth_signer = ethers.Wallet.createRandom();

    const message = `I am creating a new Squircl DID with the address ${eth_signer.address.toLowerCase()}`;

    // console.log(message);

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

    // console.log("actual_message", actual_message);
    // console.log("digest", msg_digest);

    const sig = await program.methods
      .createDidEvm(didStr, {
        addressBase58: base58.encode(arrayify("0x" + eth_address)),
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
          ethAddress: eth_address,
          message: actual_message,
          signature: signature,
          recoveryId: recoveryId,
        }),
      ])
      .rpc();

    const didAccountData = await program.account.did.fetch(didAccount);

    expect(didAccountData.did).to.equal(didStr);
    expect(didAccountData.ethAddresses.length).to.equal(1);
    expect(didAccountData.ethAddresses[0].address).to.equal(
      eth_signer.address.toLowerCase()
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
});
