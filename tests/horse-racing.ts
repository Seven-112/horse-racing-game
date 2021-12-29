import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { HorseRacing } from '../target/types/horse_racing';
import {
  PublicKey,
  SystemProgram,
  Transaction,
  SYSVAR_CLOCK_PUBKEY
} from '@solana/web3.js';

import { TOKEN_PROGRAM_ID, Token, AccountLayout } from "@solana/spl-token";
import { assert } from "chai";

describe('horse-racing', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HorseRacing as Program<HorseRacing>;

  const admin = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();
  let tokenMint = null;
  let userTokenAccount = null;
  
  const NFT_ITEM_SIZE = 32 + 1 + 1 + 4; 
  const NFT_LIST_SIZE = 2 + NFT_ITEM_SIZE * 1000;

  const BTC_FEED_PK = new PublicKey("6dbkV6QCToTk6DRfuJyrGuz18kZ4rPUSHLLLVrryWdUC");
  const SOL_FEED_PK = new PublicKey("FmAmfoyPXiA8Vhhe6MZTr3U6rZfEZ1ctEHay1ysqCqcf");

  it('Is initialized!', async () => {
    // Add your test here.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey, 10000000000),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 10000000000),
      "confirmed"
    );
    let nftListAccountPk = await PublicKey.createWithSeed(
      admin.publicKey,
      "nft_list",
      program.programId,
    );
    let ix = SystemProgram.createAccountWithSeed({
      fromPubkey: admin.publicKey,
      basePubkey: admin.publicKey,
      seed: "nft_list",
      newAccountPubkey: nftListAccountPk,
      lamports : await provider.connection.getMinimumBalanceForRentExemption(NFT_LIST_SIZE),
      space: NFT_LIST_SIZE,
      programId: program.programId,
    });

    const [operator_list_pda, operator_list_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("operator_list"))],
      program.programId
    );

    const [race_result_pda, race_result_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("race_result"))],
      program.programId
    );

    const tx = await program.rpc.initialize(
      operator_list_bump,
      race_result_bump,
      {
        accounts: {
          admin: admin.publicKey,
          nftListAccount: nftListAccountPk,
          operatorList: operator_list_pda,
          raceResult: race_result_pda,
          systemProgram: SystemProgram.programId
        },
        signers: [admin],
        instructions: [ix]
      }
    );
    console.log("Your transaction signature", tx);
  });

  it("Add Operator", async() => {
    const [operator_list_pda, operator_list_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("operator_list"))],
      program.programId
    );
    const tx = await program.rpc.addOperator(
      {
        accounts: {
          admin: admin.publicKey,
          operator: user.publicKey,
          operatorList: operator_list_pda,
        },
        signers: [admin]
      }
    );
    console.log("Your AddOperator transaction signature", tx);

    let operator_white_list = await program.account.operatorWhiteList.fetch(operator_list_pda);
    console.log("operator_white_list =", operator_white_list);
  });

  it("Mint NFT", async () => {
    let nftListAccountPk = await PublicKey.createWithSeed(
      admin.publicKey,
      "nft_list",
      program.programId,
    );

    tokenMint = await Token.createMint(
      provider.connection,
      user,
      admin.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );
    userTokenAccount = await tokenMint.createAccount(user.publicKey);

    const [upgradable_metadata_pda, upgradable_metadata_bump] = await PublicKey.findProgramAddress(
      [tokenMint.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.rpc.mintNft(
      upgradable_metadata_bump,
      {
        accounts: {
          admin: admin.publicKey,
          owner: user.publicKey,
          nftList: nftListAccountPk,
          upgradableMetadata: upgradable_metadata_pda,
          mint: tokenMint.publicKey,
          tokenAccount: userTokenAccount,
          solFeedAccount: SOL_FEED_PK,
          btcFeedAccount: BTC_FEED_PK,
          systemProgram: SystemProgram.programId,
          clock: SYSVAR_CLOCK_PUBKEY
        },
        signers: [admin, user]
      }
    );
    console.log("Your MintNFT transaction signature", tx);

    let new_metadata = await program.account.upgradableMetadata.fetch(upgradable_metadata_pda);
    console.log("new_metadata =", new_metadata);

    let nft_list = await provider.connection.getAccountInfo(nftListAccountPk);
    console.log("nft_list.data =", nft_list.data);
    
  });

  it("Upgrade NFT", async () => {
    const [upgradable_metadata_pda, upgradable_metadata_bump] = await PublicKey.findProgramAddress(
      [tokenMint.publicKey.toBuffer()],
      program.programId
    );
    const [operator_list_pda, operator_list_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("operator_list"))],
      program.programId
    );
    let nftListAccountPk = await PublicKey.createWithSeed(
      admin.publicKey,
      "nft_list",
      program.programId,
    );

    const tx = await program.rpc.upgradeNft(
      0,
      {
        accounts: {
          admin: admin.publicKey,
          owner: user.publicKey,
          nftList: nftListAccountPk,
          upgradableMetadata: upgradable_metadata_pda,
          mint: tokenMint.publicKey,
          tokenAccount: userTokenAccount,
          operatorList: operator_list_pda,
          solFeedAccount: SOL_FEED_PK,
          btcFeedAccount: BTC_FEED_PK,
          systemProgram: SystemProgram.programId,
          clock: SYSVAR_CLOCK_PUBKEY
        },
        signers: [user]
      }
    );
    console.log("Your UpgradeNFT transaction signature", tx);

    let operator_white_list = await program.account.operatorWhiteList.fetch(operator_list_pda);
    console.log("operator_white_list =", operator_white_list);

    let new_metadata = await program.account.upgradableMetadata.fetch(upgradable_metadata_pda);
    console.log("new_metadata =", new_metadata);
  });

  it("Start Race", async () => {
    const [upgradable_metadata_pda, upgradable_metadata_bump] = await PublicKey.findProgramAddress(
      [tokenMint.publicKey.toBuffer()],
      program.programId
    );
    const [operator_list_pda, operator_list_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("operator_list"))],
      program.programId
    );
    const [race_result_pda, race_result_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("race_result"))],
      program.programId
    );
    let nftListAccountPk = await PublicKey.createWithSeed(
      admin.publicKey,
      "nft_list",
      program.programId,
    );

    const tx = await program.rpc.startRace(
      {
        accounts: {
          operator: user.publicKey,
          raceResult: race_result_pda,
          nftList: nftListAccountPk,
          operatorList: operator_list_pda,
          solFeedAccount: SOL_FEED_PK,
          btcFeedAccount: BTC_FEED_PK,
          clock: SYSVAR_CLOCK_PUBKEY
        },
        signers: [user]
      }
    );
    console.log("Your Start Race transaction signature", tx);

    let operator_white_list = await program.account.operatorWhiteList.fetch(operator_list_pda);
    console.log("operator_white_list =", operator_white_list);

    let new_metadata = await program.account.upgradableMetadata.fetch(upgradable_metadata_pda);
    console.log("new_metadata =", new_metadata);
  });
});
