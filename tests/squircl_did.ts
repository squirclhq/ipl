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

  //   it("should create a new did document with a solana wallet", async () => {
  //     const didStr = generateRandomDID();

  //     const wallet = anchor.web3.Keypair.generate();

  //     // const nonce = crypto.randomBytes(32);

  //     // const message = `I am creating a new Squircl DID with the address ${wallet.publicKey.toBase58()} and nonce ${bs58.encode(
  //     //   nonce
  //     // )}`;

  //     const message = `I am creating a new Squircl DID with the address ${wallet.publicKey.toBase58()}`;

  //     console.log(message);

  //     const msgEncoded = new TextEncoder().encode(message);

  //     const signature = nacl.sign.detached(msgEncoded, wallet.secretKey);

  //     const signatureString = bs58.encode(signature);

  //     const didHex = crypto.createHash("sha256").update(didStr).digest("hex");

  //     const [didAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [Buffer.from("did"), Buffer.from(didHex, "hex")],
  //       program.programId
  //     );

  //     // console.log("nonce", nonce);
  //     console.log("signature", signatureString);
  //     console.log("didAccount", didAccount.toBase58());
  //     console.log("didHex", didHex);
  //     console.log("did", didStr);

  //     const tx = await program.methods
  //       .createDid(
  //         didStr,
  //         wallet.publicKey.toBase58(),
  //         { solana: 0 },
  //         signatureString
  //       )
  //       .accounts({
  //         payer: payer.publicKey,
  //         did: didAccount,
  //       })
  //       .preInstructions([
  //         anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
  //           units: 1000000,
  //         }),
  //       ])
  //       .rpc();

  //     console.log(tx);
  //   });

  it("should create a new did document with a ethereum wallet", async () => {
    // const ethWallet = Wallet.createRandom();

    // const msgEncoded = new TextEncoder().encode(message);

    // const signatureFull = await ethWallet.signMessage(message);

    // const signatureBytes = arrayify(signatureFull);
    // // const didHex = crypto.createHash("sha256").update(didStr).digest("hex");

    // const messageBytes = ethers.toUtf8Bytes(message);
    // const signatureBytes = arrayify(signatureFull);
    // const publicKeyBytes = arrayify(ethWallet.publicKey);

    // const tx = await program.methods
    //   .createDid(didStr, ethWallet.address, bs58.encode(signatureBytes))
    //   .accounts({
    //     payer: payer.publicKey,
    //     did: didAccount,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //   })
    //   .preInstructions([
    //     anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
    //       units: 1000000,
    //     }),
    //   ])
    //   .rpc();

    // console.log(tx);

    // const didStr = generateRandomDID();

    // const [didAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("did"), Buffer.from(didStr, "hex")],
    //   program.programId
    // );

    // console.log("didAccount", didAccount.toBase58());

    const eth_signer = ethers.Wallet.createRandom();

    const message = `I am creating a new Squircl DID with the address ${eth_signer.address}`;

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

    console.log([].slice.call(arrayify("0x" + eth_address)).length);
    console.log([].slice.call(Buffer.from(actual_message)).length);
    console.log([].slice.call(Buffer.from(signature)).length);

    const sig = await program.methods
      .createDid(
        // didStr,
        // [].slice.call(arrayify("0x" + eth_address)),
        // [].slice.call(new Uint8Array(Buffer.from(actual_message))),
        // [].slice.call(new Uint8Array(Buffer.from(signature))),
        base58.encode(arrayify("0x" + eth_address)),
        base58.encode(signature),
        base58.encode(actual_message),
        recoveryId
      )
      .accounts({
        // did: didAccount,
        ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
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
});
