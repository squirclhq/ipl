import { Program } from "@project-serum/anchor";
import crypto from "crypto";
import { SquirclDid } from "../../target/types/squircl_did";
import * as anchor from "@project-serum/anchor";

export const generateRandomDID = () => {
  const randomBytes = crypto.randomBytes(24); // 24 bytes = 48 characters
  const randomDID = randomBytes.toString("hex");
  return randomDID;
};

export const getDIDAccount = (didStr: string, program: Program<SquirclDid>) => {
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

export const getCredentialAccount = (
  issuerDidStr: string,
  subjectDidStr: string,
  credentialId: string,
  program: Program<SquirclDid>
) => {
  const issuerDidHexString = crypto
    .createHash("sha256")
    .update(issuerDidStr, "utf-8")
    .digest("hex");
  const subjectDidHexString = crypto
    .createHash("sha256")
    .update(subjectDidStr, "utf-8")
    .digest("hex");

  const issuerSeed = Uint8Array.from(Buffer.from(issuerDidHexString, "hex"));
  const subjectSeed = Uint8Array.from(Buffer.from(subjectDidHexString, "hex"));

  const [credentialAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("credential"),
      issuerSeed,
      subjectSeed,
      Buffer.from(credentialId),
    ],
    program.programId
  );

  return credentialAccount;
};
