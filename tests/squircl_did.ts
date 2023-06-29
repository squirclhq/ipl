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
import {
  addAddressEVMwithEVMController,
  addAddressEVMwithSOLController,
  addAddressSOLWithEVMController,
  addAddrsesSOLWithSOLController,
  createDIDEVM,
  createDIDSOL,
  removeAddressEVMRemover,
  removeAddressSOLRemover,
} from "./utils/instructions";

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

  const payer = anchor.workspace.SquirclDid.provider.wallet;

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

    console.log("create did evm sig", sig);

    const rawDIDAccountData = await program.provider.connection.getAccountInfo(
      didAccount
    );

    console.log(
      "raw did account data",
      rawDIDAccountData.data.toString("base64")
    );

    // convert the raw data to a did account

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
  });

  it("should create a new did document with a solana wallet", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

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
  });

  it("should not create a did if signature is invalid, eth", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

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
  });

  it("should not create a did if signature is invalid, sol", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

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
  });

  it("can add a new eth and sol address to an existing did with eth controller", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

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
      newAddressActualMessage
    );

    console.log("add address sig", sig);

    const didAccountData = await program.account.did.fetch(didAccount);

    expect(didAccountData.did).to.equal(didStr);
    expect(didAccountData.ethAddresses.length).to.equal(2);
    expect(didAccountData.ethAddresses[1].address).to.equal(
      newEthSigner.address.toLowerCase()
    );
    expect(didAccountData.ethAddresses[1].signature).to.equal(
      hexlify(newAddressFullSigBytes)
    );

    expect(didAccountData.ethAddresses[1].role).to.deep.equal({
      admin: {},
    });
    expect(didAccountData.solAddresses).to.deep.equal([]);

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
      newMessageEncoded
    );

    console.log("add address sig", sigSOL);

    const didAccountDataSOL = await program.account.did.fetch(didAccount);

    expect(didAccountDataSOL.did).to.equal(didStr);
    expect(didAccountDataSOL.solAddresses.length).to.equal(1);
    expect(didAccountDataSOL.solAddresses[0].address).to.equal(
      keypair.publicKey.toBase58()
    );
    expect(didAccountDataSOL.solAddresses[0].signature).to.equal(
      bs58.encode(newAddressSOLSignature)
    );
    expect(didAccountDataSOL.solAddresses[0].role).to.deep.equal({
      admin: {},
    });
  });

  it("can add a new eth and sol address to an existing did with sol controller", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

    const controllerKeypair = anchor.web3.Keypair.generate();

    const message = `I am creating a new Squircl DID with the address ${controllerKeypair.publicKey.toBase58()}`;

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
      payer
    );

    const newEthSigner = ethers.Wallet.createRandom();

    const newAddressMessageAsNewAddress = `I am adding myself to the Squircl DID with the address ${newEthSigner.address.toLowerCase()}`;
    const newAddressMessageAsController = `I am adding ${newEthSigner.address.toLowerCase()} to the Squircl DID with the address ${controllerKeypair.publicKey.toBase58()}`;

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
      newAddressActualMessage
    );

    console.log("add address sig", sigEVM);

    const didAccountData = await program.account.did.fetch(didAccount);

    expect(didAccountData.did).to.equal(didStr);
    expect(didAccountData.ethAddresses.length).to.equal(1);
    expect(didAccountData.ethAddresses[0].address).to.equal(
      newEthSigner.address.toLowerCase()
    );
    expect(didAccountData.ethAddresses[0].signature).to.equal(
      hexlify(newAddressFullSigBytes)
    );
    expect(didAccountData.ethAddresses[0].role).to.deep.equal({
      admin: {},
    });

    const keypair = anchor.web3.Keypair.generate();

    const newAddressMessageAsNewAddressSOL = `I am adding myself to the Squircl DID with the address ${keypair.publicKey.toBase58()}`;
    const newAddressMessageAsControllerSOL = `I am adding ${keypair.publicKey.toBase58()} to the Squircl DID with the address ${controllerKeypair.publicKey.toBase58()}`;

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
      newMessageEncoded
    );

    console.log("add address sig", sigSOL);

    const didAccountDataSOL = await program.account.did.fetch(didAccount);

    expect(didAccountDataSOL.did).to.equal(didStr);
    expect(didAccountDataSOL.solAddresses.length).to.equal(2);
    expect(didAccountDataSOL.solAddresses[1].address).to.equal(
      keypair.publicKey.toBase58()
    );
    expect(didAccountDataSOL.solAddresses[1].signature).to.equal(
      bs58.encode(newAddressSOLSignature)
    );
    expect(didAccountDataSOL.solAddresses[1].role).to.deep.equal({
      admin: {},
    });
  });

  it("can remove an eth and sol address as sol controller and eth controller", async () => {
    const didStr = generateRandomDID();

    const didAccount = getDIDAccount(didStr);

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
  });
});
