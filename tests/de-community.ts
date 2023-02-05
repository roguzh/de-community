import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { DeCommunity } from "../target/types/de_community";

describe("de-community", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DeCommunity as Program<DeCommunity>;
  const mainKeypair = anchor.web3.Keypair.generate();


  it("Is initialized!", async () => {
    const title = "Community Title";
    const description = "Community Example Description";

    const airdropTx = await anchor.getProvider().connection.requestAirdrop(mainKeypair.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await anchor.getProvider().connection.confirmTransaction(airdropTx, "confirmed");

    const communityAddress = findProgramAddressSync([
      mainKeypair.publicKey.toBuffer(),
      Buffer.from(title)
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
      title, description, null
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
});
