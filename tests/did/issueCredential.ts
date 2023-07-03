import { Program } from "@project-serum/anchor";
import { SquirclDid } from "../../target/types/squircl_did";
import {
  generateRandomDID,
  getCredentialAccount,
  getDIDAccount,
} from "../utils/pda";
import { ethers } from "ethers";
import { signEthMessage } from "../utils/signatures";
import { createDIDEVM, issueCredentialEth } from "../utils/instructions";
import nacl from "tweetnacl";
import { expect } from "chai";

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
    issuerDidStr,
    subjectDidStr,
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
    false,
    false
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
};
