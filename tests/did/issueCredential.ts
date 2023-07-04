import { Program } from "@project-serum/anchor";
import { SquirclDid } from "../../target/types/squircl_did";
import {
  generateRandomDID,
  getCredentialAccount,
  getDIDAccount,
} from "../utils/pda";
import { ethers } from "ethers";
import { signEthMessage } from "../utils/signatures";
import {
  createDIDEVM,
  createDIDSOL,
  issueCredentialEth,
  issueCredentialSol,
  updateCredentialEth,
  updateCredentialSol,
} from "../utils/instructions";
import nacl from "tweetnacl";
import { expect } from "chai";
import * as anchor from "@project-serum/anchor";

export const issueCredentialEvmTest = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const issuerDidStr = generateRandomDID();
  const subjectDidStr = generateRandomDID();

  const issuerDidAccount = getDIDAccount(issuerDidStr, program);
  const subjectDidAccount = getDIDAccount(subjectDidStr, program);

  const issuerEthSigner = ethers.Wallet.createRandom();
  const subjectEthSigner = ethers.Wallet.createRandom();

  const issuerCreateMessage = `I am creating a new Squircl DID with the address ${issuerEthSigner.address.toLowerCase()}`;
  const subjectCreateMessage = `I am creating a new Squircl DID with the address ${subjectEthSigner.address.toLowerCase()}`;

  const {
    actual_message: issuerCreateAcutalMessage,
    signature: issuerCreateSignature,
    recoveryId: issuerCreateRecoveryId,
  } = await signEthMessage(issuerCreateMessage, issuerEthSigner);

  const {
    actual_message: subjectCreateAcutalMessage,
    signature: subjectCreateSignature,
    recoveryId: subjectCreateRecoveryId,
  } = await signEthMessage(subjectCreateMessage, subjectEthSigner);

  await createDIDEVM(
    program,
    issuerDidStr,
    issuerEthSigner,
    issuerCreateSignature,
    issuerCreateRecoveryId,
    issuerDidAccount,
    issuerCreateAcutalMessage,
    payer
  );

  await createDIDEVM(
    program,
    subjectDidStr,
    subjectEthSigner,
    subjectCreateSignature,
    subjectCreateRecoveryId,
    subjectDidAccount,
    subjectCreateAcutalMessage,
    payer
  );

  const credentialId = "test-cred";
  const randomHash = "0x" + nacl.randomBytes(32).toString();
  const randomUri = "https://example.com/credentials/123";

  const credentialAccount = getCredentialAccount(
    issuerDidStr,
    subjectDidStr,
    credentialId,
    program
  );

  const issueMessage = `I am issuing a ${credentialId} credential to ${subjectDidStr} for the Squircl DID with the DID ${issuerDidStr}. Uri: ${randomUri}. Hash: ${randomHash}.`;

  const expiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

  const {
    actual_message: issueActualMessage,
    signature: issueSignature,
    recoveryId: issueRecoveryId,
  } = await signEthMessage(issueMessage, issuerEthSigner);

  await issueCredentialEth(
    program,
    issuerDidAccount,
    subjectDidAccount,
    credentialAccount,
    payer,
    issuerEthSigner,
    issueSignature,
    issueRecoveryId,
    issueActualMessage,
    credentialId,
    randomUri,
    randomHash,
    expiresAt,
    true,
    true
  );

  const credentialAccountData = await program.account.credential.fetch(
    credentialAccount
  );

  expect(credentialAccountData.issuerDid).to.equal(issuerDidStr);
  expect(credentialAccountData.subjectDid).to.equal(subjectDidStr);
  expect(credentialAccountData.credentialId).to.equal(credentialId);
  expect(credentialAccountData.uri).to.equal(randomUri);
  expect(credentialAccountData.credentialHash).to.equal(randomHash);
  expect(credentialAccountData.expiresAt.toNumber()).to.equal(expiresAt);
  expect(credentialAccountData.isMutable).to.equal(true);
  expect(credentialAccountData.isRevokable).to.equal(true);

  const updatedRandomUri = "https://example.com/credentials/456";
  const updatedRandomHash = "0x" + nacl.randomBytes(32).toString();

  const updatedMessage = `I am updating the ${credentialId} credential issued to ${subjectDidStr} by ${issuerDidStr}. New uri: ${updatedRandomUri}. New hash: ${updatedRandomHash}.`;

  const updatedExpiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 5; // 5 years

  const {
    actual_message: updatedActualMessage,
    signature: updatedSignature,
    recoveryId: updatedRecoveryId,
  } = await signEthMessage(updatedMessage, issuerEthSigner);

  await updateCredentialEth(
    program,
    issuerDidAccount,
    subjectDidAccount,
    credentialAccount,
    payer,
    issuerEthSigner,
    updatedSignature,
    updatedRecoveryId,
    updatedActualMessage,
    credentialId,
    updatedRandomUri,
    updatedRandomHash,
    updatedExpiresAt,
    true,
    true
  );

  const updatedAccountData = await program.account.credential.fetch(
    credentialAccount
  );

  expect(updatedAccountData.issuerDid).to.equal(issuerDidStr);
  expect(updatedAccountData.subjectDid).to.equal(subjectDidStr);
  expect(updatedAccountData.credentialId).to.equal(credentialId);
  expect(updatedAccountData.uri).to.equal(updatedRandomUri);
  expect(updatedAccountData.credentialHash).to.equal(updatedRandomHash);
  expect(updatedAccountData.expiresAt.toNumber()).to.equal(updatedExpiresAt);
  expect(updatedAccountData.isMutable).to.equal(true);
  expect(updatedAccountData.isRevokable).to.equal(true);
};

export const issueCredentialSolTest = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const issuerDidStr = generateRandomDID();
  const subjectDidStr = generateRandomDID();

  const issuerDidAccount = getDIDAccount(issuerDidStr, program);
  const subjectDidAccount = getDIDAccount(subjectDidStr, program);

  const issuerKeypair = anchor.web3.Keypair.generate();
  const subjectKeypair = anchor.web3.Keypair.generate();

  const issuerCreateMessage = `I am creating a new Squircl DID with the address ${issuerKeypair.publicKey.toString()}`;
  const subjectCreateMessage = `I am creating a new Squircl DID with the address ${subjectKeypair.publicKey.toString()}`;

  const issuerCreateMessageEncoded = Uint8Array.from(
    Buffer.from(issuerCreateMessage)
  );
  const subjectCreateMessageEncoded = Uint8Array.from(
    Buffer.from(subjectCreateMessage)
  );

  const issuerSignature = nacl.sign.detached(
    issuerCreateMessageEncoded,
    issuerKeypair.secretKey
  );
  const subjectSignature = nacl.sign.detached(
    subjectCreateMessageEncoded,
    subjectKeypair.secretKey
  );

  await createDIDSOL(
    program,
    issuerDidStr,
    issuerKeypair,
    issuerSignature,
    issuerCreateMessageEncoded,
    issuerDidAccount,
    payer
  );

  await createDIDSOL(
    program,
    subjectDidStr,
    subjectKeypair,
    subjectSignature,
    subjectCreateMessageEncoded,
    subjectDidAccount,
    payer
  );

  const credentialId = "test-cred";
  const randomHash = "0x" + nacl.randomBytes(32).toString();
  const randomUri = "https://example.com/credentials/123";

  const credentialAccount = getCredentialAccount(
    issuerDidStr,
    subjectDidStr,
    credentialId,
    program
  );

  const issueMessage = `I am issuing a ${credentialId} credential to ${subjectDidStr} for the Squircl DID with the DID ${issuerDidStr}. Uri: ${randomUri}. Hash: ${randomHash}.`;
  const issueMessageEncoded = Uint8Array.from(Buffer.from(issueMessage));
  const issueSignature = nacl.sign.detached(
    issueMessageEncoded,
    issuerKeypair.secretKey
  );

  const expiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

  await issueCredentialSol(
    program,
    issuerDidAccount,
    subjectDidAccount,
    credentialAccount,
    payer,
    issuerKeypair.publicKey,
    issueSignature,
    issueMessageEncoded,
    credentialId,
    randomUri,
    randomHash,
    expiresAt,
    true,
    true
  );

  const credentialAccountData = await program.account.credential.fetch(
    credentialAccount
  );

  expect(credentialAccountData.issuerDid).to.equal(issuerDidStr);
  expect(credentialAccountData.subjectDid).to.equal(subjectDidStr);
  expect(credentialAccountData.credentialId).to.equal(credentialId);
  expect(credentialAccountData.uri).to.equal(randomUri);
  expect(credentialAccountData.credentialHash).to.equal(randomHash);
  expect(credentialAccountData.expiresAt.toNumber()).to.equal(expiresAt);
  expect(credentialAccountData.isMutable).to.equal(true);
  expect(credentialAccountData.isRevokable).to.equal(true);

  const updatedRandomUri = "https://example.com/credentials/456";
  const updatedRandomHash = "0x" + nacl.randomBytes(32).toString();

  const updatedMessage = `I am updating the ${credentialId} credential issued to ${subjectDidStr} by ${issuerDidStr}. New uri: ${updatedRandomUri}. New hash: ${updatedRandomHash}.`;
  const updatedMessageEncoded = Uint8Array.from(Buffer.from(updatedMessage));

  const updatedSignature = nacl.sign.detached(
    updatedMessageEncoded,
    issuerKeypair.secretKey
  );

  const updatedExpiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 5; // 5 years

  await updateCredentialSol(
    program,
    issuerDidAccount,
    subjectDidAccount,
    credentialAccount,
    payer,
    issuerKeypair.publicKey,
    updatedSignature,
    updatedMessageEncoded,
    credentialId,
    updatedRandomUri,
    updatedRandomHash,
    updatedExpiresAt,
    true,
    true
  );

  const updatedAccountData = await program.account.credential.fetch(
    credentialAccount
  );

  expect(updatedAccountData.issuerDid).to.equal(issuerDidStr);
  expect(updatedAccountData.subjectDid).to.equal(subjectDidStr);
  expect(updatedAccountData.credentialId).to.equal(credentialId);
  expect(updatedAccountData.uri).to.equal(updatedRandomUri);
  expect(updatedAccountData.credentialHash).to.equal(updatedRandomHash);
  expect(updatedAccountData.expiresAt.toNumber()).to.equal(updatedExpiresAt);
  expect(updatedAccountData.isMutable).to.equal(true);
  expect(updatedAccountData.isRevokable).to.equal(true);
};
