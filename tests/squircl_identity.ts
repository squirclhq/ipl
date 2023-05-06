import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SquirclIdentity } from "../target/types/squircl_identity";
import lumina from "@lumina-dev/test";
import { expect } from "chai";

lumina();

describe("squircl_identity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SquirclIdentity as Program<SquirclIdentity>;

  const userKeypair = anchor.web3.Keypair.generate();
  const userAddress = userKeypair.publicKey;

  const issuerWallet = anchor.workspace.SquirclIdentity.provider.wallet;
  const issuerPublickey = issuerWallet.publicKey;

  it("can issue a credential", async () => {
    const credential_id = "g1";
    const uri = "https://example.com/credentials/" + credential_id;
    const hash = "0x" + "a".repeat(64);
    const expiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

    const [credentialPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        issuerPublickey.toBuffer(),
        userAddress.toBuffer(),
        Buffer.from(credential_id),
      ],
      program.programId
    );

    await program.methods
      .issueCredential(
        credential_id,
        uri,
        hash,
        false,
        false,
        new anchor.BN(expiresAt)
      )
      .accounts({
        credential: credentialPDA,
        issuer: issuerPublickey,
        subject: userAddress,
      })
      .rpc();

    const credentialAccount = await program.account.credential.fetch(
      credentialPDA
    );

    expect(credentialAccount.issuer.toBase58()).to.equal(
      issuerPublickey.toBase58()
    );
    expect(credentialAccount.subject.toBase58()).to.equal(
      userAddress.toBase58()
    );
    expect(credentialAccount.credentialId).to.equal(credential_id);
    expect(credentialAccount.uri).to.equal(uri);
    expect(credentialAccount.credentialHash).to.equal(hash);
    expect(credentialAccount.expiresAt.toNumber()).to.equal(expiresAt);
    expect(credentialAccount.isRevokable).to.equal(false);
    expect(credentialAccount.isMutable).to.equal(false);
  });

  it("can create a mutable credential and update it", async () => {
    const credential_id = "g2";
    const uri = "https://example.com/credentials/" + credential_id;
    const hash = "0x" + "a".repeat(64);
    const expiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

    const [credentialPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        issuerPublickey.toBuffer(),
        userAddress.toBuffer(),
        Buffer.from(credential_id),
      ],
      program.programId
    );

    await program.methods
      .issueCredential(
        credential_id,
        uri,
        hash,
        true,
        false,
        new anchor.BN(expiresAt)
      )
      .accounts({
        credential: credentialPDA,
        issuer: issuerPublickey,
        subject: userAddress,
      })
      .rpc();

    const credentialAccount = await program.account.credential.fetch(
      credentialPDA
    );

    expect(credentialAccount.issuer.toBase58()).to.equal(
      issuerPublickey.toBase58()
    );
    expect(credentialAccount.subject.toBase58()).to.equal(
      userAddress.toBase58()
    );
    expect(credentialAccount.credentialId).to.equal(credential_id);
    expect(credentialAccount.uri).to.equal(uri);
    expect(credentialAccount.credentialHash).to.equal(hash);
    expect(credentialAccount.expiresAt.toNumber()).to.equal(expiresAt);
    expect(credentialAccount.isRevokable).to.equal(false);
    expect(credentialAccount.isMutable).to.equal(true);

    const newUri = "https://example.com/credentials/" + credential_id + "/new";
    const newHash = "0x" + "b".repeat(64);

    await program.methods
      .updateCredential(
        credential_id,
        newUri,
        newHash,
        false,
        false,
        new anchor.BN(expiresAt)
      )
      .accounts({
        credential: credentialPDA,
        issuer: issuerPublickey,
        subject: userAddress,
      })
      .rpc();

    const updatedCredentialAccount = await program.account.credential.fetch(
      credentialPDA
    );

    expect(updatedCredentialAccount.uri).to.equal(newUri);
    expect(updatedCredentialAccount.credentialHash).to.equal(newHash);
    expect(updatedCredentialAccount.isRevokable).to.equal(false);
    expect(updatedCredentialAccount.isMutable).to.equal(false);
    expect(updatedCredentialAccount.expiresAt.toNumber()).to.equal(expiresAt);
  });

  it("can create a revokable credential and revoke it", async () => {
    const credential_id = "g3";
    const uri = "https://example.com/credentials/" + credential_id;
    const hash = "0x" + "a".repeat(64);
    const expiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

    const [credentialPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        issuerPublickey.toBuffer(),
        userAddress.toBuffer(),
        Buffer.from(credential_id),
      ],
      program.programId
    );

    await program.methods
      .issueCredential(
        credential_id,
        uri,
        hash,
        false,
        true,
        new anchor.BN(expiresAt)
      )
      .accounts({
        credential: credentialPDA,
        issuer: issuerPublickey,
        subject: userAddress,
      })
      .rpc();

    const credentialAccount = await program.account.credential.fetch(
      credentialPDA
    );

    expect(credentialAccount.issuer.toBase58()).to.equal(
      issuerPublickey.toBase58()
    );
    expect(credentialAccount.subject.toBase58()).to.equal(
      userAddress.toBase58()
    );
    expect(credentialAccount.credentialId).to.equal(credential_id);
    expect(credentialAccount.uri).to.equal(uri);
    expect(credentialAccount.credentialHash).to.equal(hash);
    expect(credentialAccount.expiresAt.toNumber()).to.equal(expiresAt);
    expect(credentialAccount.isRevokable).to.equal(true);
    expect(credentialAccount.isMutable).to.equal(false);

    await program.methods
      .revokeCredential(credential_id)
      .accounts({
        credential: credentialPDA,
        issuer: issuerPublickey,
        subject: userAddress,
      })
      .rpc();

    try {
      await program.account.credential.fetch(credentialPDA);
    } catch (e) {
      expect(e.message).to.contain("Account does not exist");
    }
  });

  it("cannot update an immutable credential", async () => {
    const credential_id = "g4";
    const uri = "https://example.com/credentials/" + credential_id;
    const hash = "0x" + "a".repeat(64);
    const expiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

    const [credentialPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        issuerPublickey.toBuffer(),
        userAddress.toBuffer(),
        Buffer.from(credential_id),
      ],
      program.programId
    );

    await program.methods
      .issueCredential(
        credential_id,
        uri,
        hash,
        false,
        false,
        new anchor.BN(expiresAt)
      )
      .accounts({
        credential: credentialPDA,
        issuer: issuerPublickey,
        subject: userAddress,
      })
      .rpc();

    const credentialAccount = await program.account.credential.fetch(
      credentialPDA
    );

    expect(credentialAccount.issuer.toBase58()).to.equal(
      issuerPublickey.toBase58()
    );
    expect(credentialAccount.subject.toBase58()).to.equal(
      userAddress.toBase58()
    );
    expect(credentialAccount.credentialId).to.equal(credential_id);
    expect(credentialAccount.uri).to.equal(uri);
    expect(credentialAccount.credentialHash).to.equal(hash);
    expect(credentialAccount.expiresAt.toNumber()).to.equal(expiresAt);
    expect(credentialAccount.isRevokable).to.equal(false);
    expect(credentialAccount.isMutable).to.equal(false);

    const newUri = "https://example.com/credentials/" + credential_id + "/new";
    const newHash = "0x" + "b".repeat(64);

    try {
      await program.methods
        .updateCredential(
          credential_id,
          newUri,
          newHash,
          false,
          false,
          new anchor.BN(expiresAt)
        )
        .accounts({
          credential: credentialPDA,
          issuer: issuerPublickey,
          subject: userAddress,
        })
        .rpc();
    } catch (e) {
      expect(e.message).to.contain("Error Code: CredentialIsNotMutable.");
    }
  });

  it("cannot revoke an irrevokable credential", async () => {
    const credential_id = "g5";
    const uri = "https://example.com/credentials/" + credential_id;
    const hash = "0x" + "a".repeat(64);
    const expiresAt = new Date().getTime() + 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

    const [credentialPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        issuerPublickey.toBuffer(),
        userAddress.toBuffer(),
        Buffer.from(credential_id),
      ],
      program.programId
    );

    await program.methods
      .issueCredential(
        credential_id,
        uri,
        hash,
        false,
        false,
        new anchor.BN(expiresAt)
      )
      .accounts({
        credential: credentialPDA,
        issuer: issuerPublickey,
        subject: userAddress,
      })
      .rpc();

    const credentialAccount = await program.account.credential.fetch(
      credentialPDA
    );

    expect(credentialAccount.issuer.toBase58()).to.equal(
      issuerPublickey.toBase58()
    );
    expect(credentialAccount.subject.toBase58()).to.equal(
      userAddress.toBase58()
    );
    expect(credentialAccount.credentialId).to.equal(credential_id);
    expect(credentialAccount.uri).to.equal(uri);
    expect(credentialAccount.credentialHash).to.equal(hash);
    expect(credentialAccount.expiresAt.toNumber()).to.equal(expiresAt);
    expect(credentialAccount.isRevokable).to.equal(false);
    expect(credentialAccount.isMutable).to.equal(false);

    try {
      await program.methods
        .revokeCredential(credential_id)
        .accounts({
          credential: credentialPDA,
          issuer: issuerPublickey,
          subject: userAddress,
        })
        .rpc();
    } catch (e) {
      expect(e.message).to.contain("Error Code: CredentialIsNotRevokable.");
    }
  });
});
