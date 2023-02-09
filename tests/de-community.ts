import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { DeCommunity } from "../target/types/de_community";

describe("de-community", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DeCommunity as Program<DeCommunity>;
  const mainKeypair = anchor.web3.Keypair.generate();
  const memberKeypair = anchor.web3.Keypair.generate();

  const communityTitle = "Community Title";
  const communityDescription = "Community Example Description";


  it("Is initialized!", async () => {

    const airdropTx = await anchor.getProvider().connection.requestAirdrop(mainKeypair.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await anchor.getProvider().connection.confirmTransaction(airdropTx, "confirmed");

    const communityAddress = findProgramAddressSync([
      mainKeypair.publicKey.toBuffer(),
      Buffer.from(communityTitle)
    ], program.programId)[0];

    const proposerAddress = findProgramAddressSync([
      Buffer.from("proposer"),
      communityAddress.toBuffer(),
      mainKeypair.publicKey.toBuffer(),
    ], program.programId)[0];

    const memberAddress = findProgramAddressSync([
      Buffer.from("member"),
      communityAddress.toBuffer(),
      mainKeypair.publicKey.toBuffer(),
    ], program.programId)[0];

    const tx = await program.methods.initializeCommunity(
      communityTitle, communityDescription, new anchor.BN(86400), null
    ).accounts({
      community: communityAddress,
      initializerProposer: proposerAddress,
      initializerMember: memberAddress,
      initializer: mainKeypair.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([
      mainKeypair
    ]).rpc();

    console.log(`\tSignature: ${tx}`);

    const community = await program.account["community"].fetch(communityAddress);
    const proposer = await program.account["proposer"].fetch(proposerAddress);
    const member = await program.account["member"].fetch(memberAddress);

    console.log(community);
    console.log(proposer);
    console.log(member);
  });

  it("Initialize Member Account ", async () => {
    const communityAddress = findProgramAddressSync([
      mainKeypair.publicKey.toBuffer(),
      Buffer.from(communityTitle)
    ], program.programId)[0];

    const memberAddress = findProgramAddressSync([
      Buffer.from("member"),
      communityAddress.toBuffer(),
      memberKeypair.publicKey.toBuffer(),
    ], program.programId)[0];

    const tx = await program.methods.initializeMember()
      .accounts({
        community: communityAddress,
        member: memberAddress,
        memberOwner: memberKeypair.publicKey,
        payer: mainKeypair.publicKey
      }).signers([
        mainKeypair
      ]).rpc();

    const member = await program.account["member"].fetch(memberAddress);
    console.log(member);
  });

  it("Creating Proposal", async () => {
    const communityAddress = findProgramAddressSync([
      mainKeypair.publicKey.toBuffer(),
      Buffer.from(communityTitle)
    ], program.programId)[0];

    const proposerAddress = findProgramAddressSync([
      Buffer.from("proposer"),
      communityAddress.toBuffer(),
      mainKeypair.publicKey.toBuffer(),
    ], program.programId)[0];
    let proposer = await program.account["proposer"].fetch(proposerAddress);

    const proposalAddress = findProgramAddressSync([
      proposerAddress.toBuffer(),
      proposer.proposalCount.toBuffer('be', 8)
    ], program.programId)[0];

    const memberAddress = findProgramAddressSync([
      Buffer.from("member"),
      communityAddress.toBuffer(),
      memberKeypair.publicKey.toBuffer(),
    ], program.programId)[0];

    const tx = await program.methods.createProposal(
      {
        "manageMember": {
          "action": { "addMember": {} },
          "voted_account": memberAddress,
          "voted_account_owner": memberKeypair.publicKey
        }
      },
      new anchor.BN(1685895001)
    ).accounts({
      proposal: proposalAddress,
      proposer: proposerAddress,
      community: communityAddress,
      signer: mainKeypair.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([
      mainKeypair
    ]).rpc({
      "commitment": "confirmed"
    });
    await anchor.getProvider().connection.confirmTransaction(tx);

    const community = await program.account["community"].fetch(communityAddress);
    proposer = await program.account["proposer"].fetch(proposerAddress);
    const member = await program.account["member"].fetch(memberAddress);
    const proposal = await program.account["proposal"].fetch(proposalAddress);

    // console.log("Community");
    // console.log(community);
    // console.log("\nProposer");
    // console.log(proposer);
    // console.log("\nMember");
    // console.log(member);
    // console.log("\nProposal");
    // console.log(proposal);
    console.log("\nProposal Type");
    console.log(proposal.proposalType);
  });

});
