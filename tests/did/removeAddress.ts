import { Program } from "@project-serum/anchor";
import { SquirclDid } from "../../target/types/squircl_did";
import { generateRandomDID, getDIDAccount } from "../utils/pda";
import { ethers } from "ethers";
import { signEthMessage } from "../utils/signatures";
import {
  addAddressEVMwithEVMController,
  addAddressSOLWithEVMController,
  createDIDEVM,
  removeAddressEVMRemover,
  removeAddressSOLRemover,
} from "../utils/instructions";
import { expect } from "chai";
import nacl from "tweetnacl";
import * as anchor from "@project-serum/anchor";

export const removeAddressTest = async (
  program: Program<SquirclDid>,
  payer: any
) => {
  const didStr = generateRandomDID();

  const didAccount = getDIDAccount(didStr, program);

  const ethSigner = ethers.Wallet.createRandom();

  const message = `I am creating a new Squircl DID with the address ${ethSigner.address.toLowerCase()}`;

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
    payer
  );

  const newEthSigner = ethers.Wallet.createRandom();

  const newAddressMessageAsNewAddress = `I am adding myself to the Squircl DID with the address ${newEthSigner.address.toLowerCase()}`;
  const newAddressMessageAsController = `I am adding ${newEthSigner.address.toLowerCase()} to the Squircl DID with the address ${ethSigner.address.toLowerCase()}`;

  const {
    actual_message: newAddressActualMessage,
    signature: newAddressSignature,
    recoveryId: newAddressRecoveryId,
  } = await signEthMessage(newAddressMessageAsNewAddress, newEthSigner);

  const {
    actual_message: controllerActualMessage,
    signature: controllerSignature,
    recoveryId: controllerRecoveryId,
  } = await signEthMessage(newAddressMessageAsController, ethSigner);

  await addAddressEVMwithEVMController(
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
    newAddressActualMessage
  );

  const removeAddressEVMAsEVMControllerMessage = `I am removing ${newEthSigner.address.toLowerCase()} from the Squircl DID with the address ${ethSigner.address.toLowerCase()}`;

  const {
    actual_message: removeAddressEVMAsEVMControllerActualMessage,
    signature: removeAddressEVMAsEVMControllerSignature,
    recoveryId: removeAddressEVMAsEVMControllerRecoveryId,
  } = await signEthMessage(removeAddressEVMAsEVMControllerMessage, ethSigner);

  await removeAddressEVMRemover(
    program,
    didStr,
    didAccount,
    payer,
    ethSigner,
    removeAddressEVMAsEVMControllerSignature,
    removeAddressEVMAsEVMControllerRecoveryId,
    removeAddressEVMAsEVMControllerActualMessage,
    newEthSigner.address.toLowerCase(),
    { evm: {} }
  );

  const didAccountData = await program.account.did.fetch(didAccount);

  expect(didAccountData.did).to.equal(didStr);
  expect(didAccountData.ethAddresses.length).to.equal(1);

  const {
    actual_message: newAddressActualMessage2,
    signature: newAddressSignature2,
    recoveryId: newAddressRecoveryId2,
  } = await signEthMessage(newAddressMessageAsNewAddress, newEthSigner);

  const {
    actual_message: controllerActualMessage2,
    signature: controllerSignature2,
    recoveryId: controllerRecoveryId2,
  } = await signEthMessage(newAddressMessageAsController, ethSigner);

  await addAddressEVMwithEVMController(
    program,
    didStr,
    didAccount,
    payer,
    ethSigner,
    newEthSigner,
    controllerSignature2,
    controllerRecoveryId2,
    newAddressSignature2,
    newAddressRecoveryId2,
    controllerActualMessage2,
    newAddressActualMessage2
  );

  const removeAddressEVMAsSelfMessage = `I am removing myself from the Squircl DID with the address ${newEthSigner.address.toLowerCase()}`;

  const {
    actual_message: removeAddressEVMAsSelfActualMessage,
    signature: removeAddressEVMAsSelfSignature,
    recoveryId: removeAddressEVMAsSelfRecoveryId,
  } = await signEthMessage(removeAddressEVMAsSelfMessage, newEthSigner);

  await removeAddressEVMRemover(
    program,
    didStr,
    didAccount,
    payer,
    newEthSigner,
    removeAddressEVMAsSelfSignature,
    removeAddressEVMAsSelfRecoveryId,
    removeAddressEVMAsSelfActualMessage,
    newEthSigner.address.toLowerCase(),
    { evm: {} }
  );

  const didAccountData2 = await program.account.did.fetch(didAccount);

  expect(didAccountData2.did).to.equal(didStr);
  expect(didAccountData2.ethAddresses.length).to.equal(1);

  // add a sol address and remove it (both with eth controller signing it and sol address signing it)

  const keypair = anchor.web3.Keypair.generate();

  const newAddressMessageAsNewAddressSOL = `I am adding myself to the Squircl DID with the address ${keypair.publicKey.toBase58()}`;
  const newAddressMessageAsControllerSOL = `I am adding ${keypair.publicKey.toBase58()} to the Squircl DID with the address ${ethSigner.address.toLocaleLowerCase()}`;

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

  await addAddressSOLWithEVMController(
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
    newMessageEncoded
  );

  const removeAddressSOLAsEVMControllerMessage = `I am removing ${keypair.publicKey.toBase58()} from the Squircl DID with the address ${ethSigner.address.toLowerCase()}`;

  const {
    actual_message: removeAddressSOLAsEVMControllerActualMessage,
    signature: removeAddressSOLAsEVMControllerSignature,
    recoveryId: removeAddressSOLAsEVMControllerRecoveryId,
  } = await signEthMessage(removeAddressSOLAsEVMControllerMessage, ethSigner);

  await removeAddressEVMRemover(
    program,
    didStr,
    didAccount,
    payer,
    ethSigner,
    removeAddressSOLAsEVMControllerSignature,
    removeAddressSOLAsEVMControllerRecoveryId,
    removeAddressSOLAsEVMControllerActualMessage,
    keypair.publicKey.toBase58(),
    { sol: {} }
  );

  const didAccountData3 = await program.account.did.fetch(didAccount);

  expect(didAccountData3.did).to.equal(didStr);
  expect(didAccountData3.ethAddresses.length).to.equal(1);
  expect(didAccountData3.solAddresses.length).to.equal(0);

  const newMessageEncoded2 = Uint8Array.from(
    Buffer.from(newAddressMessageAsNewAddressSOL)
  );

  const {
    actual_message: controllerActualMessageSOL2,
    signature: controllerSignatureSOL2,
    recoveryId: controllerRecoveryIdSOL2,
  } = await signEthMessage(newAddressMessageAsControllerSOL, ethSigner);

  const newAddressSOLSignature2 = nacl.sign.detached(
    newMessageEncoded,
    keypair.secretKey
  );

  await addAddressSOLWithEVMController(
    program,
    didStr,
    didAccount,
    payer,
    ethSigner,
    controllerSignatureSOL2,
    controllerRecoveryIdSOL2,
    controllerActualMessageSOL2,
    keypair.publicKey,
    newAddressSOLSignature2,
    newMessageEncoded2
  );

  const removeAddressSOLAsSelfMessage = `I am removing myself from the Squircl DID with the address ${keypair.publicKey.toBase58()}`;

  console.log(removeAddressSOLAsSelfMessage);

  const removeAddressSOLAsSelfMessageEncoded = Uint8Array.from(
    Buffer.from(removeAddressSOLAsSelfMessage)
  );

  const removeAddressSOLAsSelfSignature = nacl.sign.detached(
    removeAddressSOLAsSelfMessageEncoded,
    keypair.secretKey
  );

  await removeAddressSOLRemover(
    program,
    didStr,
    didAccount,
    payer,
    keypair.publicKey,
    removeAddressSOLAsSelfSignature,
    removeAddressSOLAsSelfMessageEncoded,
    keypair.publicKey.toBase58(),
    { sol: {} }
  );

  const didAccountData4 = await program.account.did.fetch(didAccount);

  expect(didAccountData4.did).to.equal(didStr);
  expect(didAccountData4.ethAddresses.length).to.equal(1);
  expect(didAccountData4.solAddresses.length).to.equal(0);
};
