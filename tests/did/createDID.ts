import { Program } from "@project-serum/anchor";
import { generateRandomDID, getDIDAccount } from "../utils/pda";
import { SquirclDid } from "../../target/types/squircl_did";
import { ethers, hexlify } from "ethers";
import { signEthMessage } from "../utils/signatures";
import { createDIDEVM, createDIDSOL } from "../utils/instructions";
import { expect } from "chai";
import bs58 from "bs58";
import nacl from "tweetnacl";
import * as anchor from "@project-serum/anchor";

export const createDIDEthereumTest = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const didStr = generateRandomDID();

  const didAccount = getDIDAccount(didStr, program);

  const ethSigner = ethers.Wallet.createRandom();

  const message = `I am creating a new Squircl DID with the address ${ethSigner.address.toLowerCase()}`;

  const { actual_message, signature, recoveryId, full_sig_bytes } =
    await signEthMessage(message, ethSigner);

  const sig = await createDIDEVM(
    program,
    didStr,
    ethSigner,
    signature,
    recoveryId,
    didAccount,
    actual_message,
    payer
  );

  const rawDIDAccountData = await program.provider.connection.getAccountInfo(
    didAccount
  );

  console.log(
    "raw did account data",
    rawDIDAccountData.data.toString("base64")
  );

  // convert the raw data to a did account

  console.log(didAccount.toBase58());

  console.log(program.coder.accounts.decode("Did", rawDIDAccountData.data));

  const didAccountData = await program.account.did.fetch(didAccount);

  console.log("did account data", didAccountData);

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
};

export const createDIDSolTest = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const didStr = generateRandomDID();

  const didAccount = getDIDAccount(didStr, program);

  const keypair = anchor.web3.Keypair.generate();
  const message = `I am creating a new Squircl DID with the address ${keypair.publicKey.toBase58()}`;

  const messageEncoded = Uint8Array.from(Buffer.from(message));

  const signature = nacl.sign.detached(messageEncoded, keypair.secretKey);

  const sig = await createDIDSOL(
    program,
    didStr,
    keypair,
    signature,
    messageEncoded,
    didAccount,
    payer
  );

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
};

export const createDIDEvmTestInvalidSig = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const didStr = generateRandomDID();

  const didAccount = getDIDAccount(didStr, program);

  const ethSigner = ethers.Wallet.createRandom();

  const message = "random fake message, will cause invalid signature";

  const { actual_message, signature, recoveryId } = await signEthMessage(
    message,
    ethSigner
  );

  try {
    await createDIDEVM(
      program,
      didStr,
      ethSigner,
      signature,
      recoveryId,
      didAccount,
      actual_message,
      payer
    );
  } catch (e) {
    expect(e.toString()).to.equal(
      "AnchorError occurred. Error Code: InvalidSignature. Error Number: 6001. Error Message: Invalid signature."
    );
  }
};

export const createDIDSolTestInvalidSig = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const didStr = generateRandomDID();

  const didAccount = getDIDAccount(didStr, program);

  const keypair = anchor.web3.Keypair.generate();

  const message = "random fake message, will cause invalid signature";
  const messageEncoded = Uint8Array.from(Buffer.from(message));

  const signature = nacl.sign.detached(messageEncoded, keypair.secretKey);

  try {
    await createDIDSOL(
      program,
      didStr,
      keypair,
      signature,
      messageEncoded,
      didAccount,
      payer
    );
  } catch (e) {
    expect(e.toString()).to.equal(
      "AnchorError occurred. Error Code: InvalidSignature. Error Number: 6001. Error Message: Invalid signature."
    );
  }
};
