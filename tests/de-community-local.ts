import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { DeCommunity, IDL as DeCommunityIDL } from "../target/types/de_community";

const PROGRAM_ID = new anchor.web3.PublicKey("AHJ8w9ePSzS7mjhXV4LAqatQtdEk9DpTgTLdzrpHU6P2");
const mainKeypair = anchor.web3.Keypair.generate();
const memberKeypair = anchor.web3.Keypair.generate();

(async () => {
    const connection = new anchor.web3.Connection("http://127.0.0.1:8899", "confirmed");
    const provider = new anchor.AnchorProvider(connection, null as any, {});
    const program = new Program<DeCommunity>(
        DeCommunityIDL,
        PROGRAM_ID,
        provider
    );

    const communityTitle = "Community Title";
    const communityDescription = "Community Example Description";

    const airdropTx = await connection.requestAirdrop(mainKeypair.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropTx, "confirmed");
    console.log(`Airdrop Signature: ${airdropTx}`);

    const communityAddress = findProgramAddressSync([
        mainKeypair.publicKey.toBuffer(),
        Buffer.from(communityTitle)
    ], program.programId)[0];

    const proposerAddress = findProgramAddressSync([
        Buffer.from("proposer"),
        communityAddress.toBuffer(),
        mainKeypair.publicKey.toBuffer(),
    ], program.programId)[0];
    // let proposer = await program.account["proposer"].fetch(proposerAddress);

    const mainMemberAddress = findProgramAddressSync([
        Buffer.from("member"),
        communityAddress.toBuffer(),
        mainKeypair.publicKey.toBuffer(),
    ], program.programId)[0];


    const initCommunityIx = await program.methods.initializeCommunity(
        communityTitle, communityDescription, new anchor.BN(86400), null
    ).accounts({
        community: communityAddress,
        initializerProposer: proposerAddress,
        initializerMember: mainMemberAddress,
        initializer: mainKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
    }).signers([
        mainKeypair
    ]).instruction();
    const initCommunitySignature = await sendTransactionAndConfirm(initCommunityIx, connection);

    const memberAddress = findProgramAddressSync([
        Buffer.from("member"),
        communityAddress.toBuffer(),
        memberKeypair.publicKey.toBuffer(),
    ], program.programId)[0];

    const initMemberIx = await program.methods.initializeMember()
        .accounts({
            community: communityAddress,
            member: memberAddress,
            memberOwner: memberKeypair.publicKey,
            payer: mainKeypair.publicKey
        }).signers([
            mainKeypair
        ]).instruction();
    const initMemberSignature = await sendTransactionAndConfirm(initMemberIx, connection);

    let proposer = await program.account["proposer"].fetch(proposerAddress);
    const proposalAddress = findProgramAddressSync([
        proposerAddress.toBuffer(),
        proposer.proposalCount.toBuffer('be', 8)
      ], program.programId)[0];

    const createProposalIx = await program.methods.createProposal(
        {
          "manageMember": {
            "action": { "addMember": {} },
            "voted_account": memberAddress,
            "voted_account_owner": memberKeypair.publicKey
          }
        },
        // {
        //     "custom": {
        //         "title": "Test Title",
        //         "description": "Test Description"
        //     }
        // },
        new anchor.BN(1685895001)
      ).accounts({
        proposal: proposalAddress,
        proposer: proposerAddress,
        community: communityAddress,
        signer: mainKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        votedMember: memberAddress,
        votedMemberOwner: memberKeypair.publicKey
      }).signers([
        mainKeypair
      ]).instruction();
      const createProposalSignature = await sendTransactionAndConfirm(createProposalIx, connection);

    const community = await program.account["community"].fetch(communityAddress);
    proposer = await program.account["proposer"].fetch(proposerAddress);
    const member = await program.account["member"].fetch(memberAddress);
    const proposal = await program.account["proposal"].fetch(proposalAddress);

    console.log("\nProposal Type");
    console.log(proposal.proposalType);
})();



async function sendTransactionAndConfirm(instruction: anchor.web3.TransactionInstruction, connection: anchor.web3.Connection) {
    const tx = new anchor.web3.Transaction().add(instruction);
    const recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    tx.feePayer = mainKeypair.publicKey;
    tx.recentBlockhash = recentBlockhash;

    const signature = await connection.sendTransaction(
        tx,
        [mainKeypair]
    );
    await connection.confirmTransaction(signature, 'confirmed');
    return signature;
}