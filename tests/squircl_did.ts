import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SquirclDid } from "../target/types/squircl_did";
import lumina from "@lumina-dev/test";

import {
  createDIDEthereumTest,
  createDIDEvmTestInvalidSig,
  createDIDSolTest,
  createDIDSolTestInvalidSig,
} from "./did/createDID";
import { addAddressEth, addAddressSol } from "./did/addAddress";
import { removeAddressTest } from "./did/removeAddress";
import {
  issueCredentialEvmTest,
  issueCredentialSolTest,
} from "./did/issueCredential";

lumina();

describe("squircl_identity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SquirclDid as Program<SquirclDid>;

  const payer = anchor.workspace.SquirclDid.provider.wallet;

  // it("should create a new did document with a ethereum wallet", async () => {
  //   await createDIDEthereumTest(program, payer);
  // });

  // it("should create a new did document with a solana wallet", async () => {
  //   await createDIDSolTest(program, payer);
  // });

  // it("should not create a did if signature is invalid, eth", async () => {
  //   await createDIDEvmTestInvalidSig(program, payer);
  // });

  // it("should not create a did if signature is invalid, sol", async () => {
  //   await createDIDSolTestInvalidSig(program, payer);
  // });

  it("can add a new eth and sol address to an existing did with eth controller", async () => {
    await addAddressEth(program, payer);
  });

  // it("can add a new eth and sol address to an existing did with sol controller", async () => {
  //   await addAddressSol(program, payer);
  // });

  it("can remove an eth and sol address as sol controller and eth controller", async () => {
    await removeAddressTest(program, payer);
  });

  // it("can issue a credential evm", async () => {
  //   await issueCredentialEvmTest(program, payer);
  // });

  // it("can issue a credential sol", async () => {
  //   await issueCredentialSolTest(program, payer);
  // });
});
