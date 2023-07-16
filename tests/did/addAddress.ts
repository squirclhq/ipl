import { Program } from "@project-serum/anchor";
import { SquirclDid } from "../../target/types/squircl_did";
import { generateRandomDID, getDIDAccount } from "../utils/pda";
import { ethers, hexlify } from "ethers";
import { signEthMessage } from "../utils/signatures";
import {
  addAddressEVMwithEVMController,
  addAddressEVMwithSOLController,
  addAddressSOLWithEVMController,
  addAddrsesSOLWithSOLController,
  createDIDEVM,
  createDIDSOL,
} from "../utils/instructions";
import { expect } from "chai";
import nacl from "tweetnacl";
import bs58 from "bs58";
import * as anchor from "@project-serum/anchor";

export const addAddressEth = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const nonce = Math.floor(Date.now() / 1000);

  const didStr = generateRandomDID();

  const didAccount = getDIDAccount(didStr, program);

  const ethSigner = ethers.Wallet.createRandom();

  const message = `I am creating a new Squircl DID with the address ${ethSigner.address.toLowerCase()}. Nonce: ${nonce}`;

  const { actual_message, signature, recoveryId } = await signEthMessage(
    message,
    ethSigner
  );

  await createDIDEVM(
    program,
    didStr,
    ethSigner,
    signature,
    recoveryId,
    didAccount,
    actual_message,
    payer,
    nonce
  );

  const newEthSigner = ethers.Wallet.createRandom();

  const newAddressMessageAsNewAddress = `I am adding myself to the Squircl DID with the address ${newEthSigner.address.toLowerCase()}. Nonce: ${nonce}`;
  const newAddressMessageAsController = `I am adding ${newEthSigner.address.toLowerCase()} to the Squircl DID with the address ${ethSigner.address.toLowerCase()}. Nonce: ${nonce}`;

  const {
    actual_message: newAddressActualMessage,
    signature: newAddressSignature,
    recoveryId: newAddressRecoveryId,
    full_sig_bytes: newAddressFullSigBytes,
  } = await signEthMessage(newAddressMessageAsNewAddress, newEthSigner);

  const {
    actual_message: controllerActualMessage,
    signature: controllerSignature,
    recoveryId: controllerRecoveryId,
  } = await signEthMessage(newAddressMessageAsController, ethSigner);

  const sig = await addAddressEVMwithEVMController(
    program,
    didStr,
    didAccount,
    payer,
    ethSigner,
    newEthSigner,
    controllerSignature,
    controllerRecoveryId,
    newAddressSignature,
    newAddressRecoveryId,
    controllerActualMessage,
    newAddressActualMessage,
    nonce
  );

  // console.log("add address sig", sig);

  const didAccountData = await program.account.did.fetch(didAccount);

  expect(didAccountData.did).to.equal(didStr);
  expect(didAccountData.ethAddresses.length).to.equal(2);
  expect(didAccountData.ethAddresses[1].address).to.equal(
    newEthSigner.address.toLowerCase()
  );

  expect(didAccountData.ethAddresses[1].role).to.deep.equal({
    admin: {},
  });
  expect(didAccountData.solAddresses).to.deep.equal([]);

  const keypair = anchor.web3.Keypair.generate();

  const newAddressMessageAsNewAddressSOL = `I am adding myself to the Squircl DID with the address ${keypair.publicKey.toBase58()}. Nonce: ${nonce}`;
  const newAddressMessageAsControllerSOL = `I am adding ${keypair.publicKey.toBase58()} to the Squircl DID with the address ${ethSigner.address.toLocaleLowerCase()}. Nonce: ${nonce}`;

  const newMessageEncoded = Uint8Array.from(
    Buffer.from(newAddressMessageAsNewAddressSOL)
  );

  const {
    actual_message: controllerActualMessageSOL,
    signature: controllerSignatureSOL,
    recoveryId: controllerRecoveryIdSOL,
  } = await signEthMessage(newAddressMessageAsControllerSOL, ethSigner);

  const newAddressSOLSignature = nacl.sign.detached(
    newMessageEncoded,
    keypair.secretKey
  );

  const sigSOL = await addAddressSOLWithEVMController(
    program,
    didStr,
    didAccount,
    payer,
    ethSigner,
    controllerSignatureSOL,
    controllerRecoveryIdSOL,
    controllerActualMessageSOL,
    keypair.publicKey,
    newAddressSOLSignature,
    newMessageEncoded,
    nonce
  );

  // console.log("add address sig", sigSOL);

  const didAccountDataSOL = await program.account.did.fetch(didAccount);

  expect(didAccountDataSOL.did).to.equal(didStr);
  expect(didAccountDataSOL.solAddresses.length).to.equal(1);
  expect(didAccountDataSOL.solAddresses[0].address).to.equal(
    keypair.publicKey.toBase58()
  );

  expect(didAccountDataSOL.solAddresses[0].role).to.deep.equal({
    admin: {},
  });
};

export const addAddressSol = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const nonce = Math.floor(Date.now() / 1000);

  const didStr = generateRandomDID();

  const didAccount = getDIDAccount(didStr, program);

  const controllerKeypair = anchor.web3.Keypair.generate();

  const message = `I am creating a new Squircl DID with the address ${controllerKeypair.publicKey.toBase58()}. Nonce: ${nonce}`;

  const messageEncoded = Uint8Array.from(Buffer.from(message));

  const controllerSignature = nacl.sign.detached(
    messageEncoded,
    controllerKeypair.secretKey
  );

  await createDIDSOL(
    program,
    didStr,
    controllerKeypair,
    controllerSignature,
    messageEncoded,
    didAccount,
    payer,
    nonce
  );

  const newEthSigner = ethers.Wallet.createRandom();

  const newAddressMessageAsNewAddress = `I am adding myself to the Squircl DID with the address ${newEthSigner.address.toLowerCase()}. Nonce: ${nonce}`;
  const newAddressMessageAsController = `I am adding ${newEthSigner.address.toLowerCase()} to the Squircl DID with the address ${controllerKeypair.publicKey.toBase58()}. Nonce: ${nonce}`;

  const newAddressMessageAsControllerEncoded = Uint8Array.from(
    Buffer.from(newAddressMessageAsController)
  );

  const controllerSignatureEVM = nacl.sign.detached(
    newAddressMessageAsControllerEncoded,
    controllerKeypair.secretKey
  );

  const {
    actual_message: newAddressActualMessage,
    signature: newAddressSignature,
    recoveryId: newAddressRecoveryId,
    full_sig_bytes: newAddressFullSigBytes,
  } = await signEthMessage(newAddressMessageAsNewAddress, newEthSigner);

  const sigEVM = await addAddressEVMwithSOLController(
    program,
    didStr,
    didAccount,
    payer,
    controllerKeypair.publicKey,
    controllerSignatureEVM,
    newAddressMessageAsControllerEncoded,
    newEthSigner,
    newAddressSignature,
    newAddressRecoveryId,
    newAddressActualMessage,
    nonce
  );

  // console.log("add address sig", sigEVM);

  const didAccountData = await program.account.did.fetch(didAccount);

  expect(didAccountData.did).to.equal(didStr);
  expect(didAccountData.ethAddresses.length).to.equal(1);
  expect(didAccountData.ethAddresses[0].address).to.equal(
    newEthSigner.address.toLowerCase()
  );

  expect(didAccountData.ethAddresses[0].role).to.deep.equal({
    admin: {},
  });

  const keypair = anchor.web3.Keypair.generate();

  const newAddressMessageAsNewAddressSOL = `I am adding myself to the Squircl DID with the address ${keypair.publicKey.toBase58()}. Nonce: ${nonce}`;
  const newAddressMessageAsControllerSOL = `I am adding ${keypair.publicKey.toBase58()} to the Squircl DID with the address ${controllerKeypair.publicKey.toBase58()}. Nonce: ${nonce}`;

  const newMessageEncoded = Uint8Array.from(
    Buffer.from(newAddressMessageAsNewAddressSOL)
  );

  const newAddressSOLSignature = nacl.sign.detached(
    newMessageEncoded,
    keypair.secretKey
  );

  const newAddressMessageAsControllerSOLEncoded = Uint8Array.from(
    Buffer.from(newAddressMessageAsControllerSOL)
  );

  const controllerSignatureSOL = nacl.sign.detached(
    newAddressMessageAsControllerSOLEncoded,
    controllerKeypair.secretKey
  );

  const sigSOL = await addAddrsesSOLWithSOLController(
    program,
    didStr,
    didAccount,
    payer,
    controllerKeypair.publicKey,
    controllerSignatureSOL,
    newAddressMessageAsControllerSOLEncoded,
    keypair.publicKey,
    newAddressSOLSignature,
    newMessageEncoded,
    nonce
  );

  // console.log("add address sig", sigSOL);

  const didAccountDataSOL = await program.account.did.fetch(didAccount);

  expect(didAccountDataSOL.did).to.equal(didStr);
  expect(didAccountDataSOL.solAddresses.length).to.equal(2);
  expect(didAccountDataSOL.solAddresses[1].address).to.equal(
    keypair.publicKey.toBase58()
  );

  expect(didAccountDataSOL.solAddresses[1].role).to.deep.equal({
    admin: {},
  });
};
