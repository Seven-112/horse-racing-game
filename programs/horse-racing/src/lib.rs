    use anchor_lang::prelude::*;
    use anchor_spl::token::{self, TokenAccount};
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        program_error::ProgramError,
        sysvar::{clock::Clock},
        program::{invoke},
        msg
    };
    declare_id!("DBrWpEKgTccMoAQ6umLTSUWsfLR8wYNmvHJSUPGe4XJ8");

    pub const UPGRADABLE_METASIZE: usize = 1 + 1 + 1;
    pub const NFT_LIST_SIZE: usize = 2 + (32 + 2) * 1000; // 2 bytes for nft count
    pub const OPERATOR_LIST_SIZE: usize = 32 * 10 + 2;
    pub const MAX_ADMIN_CNT: usize = 10;
    pub const RACE_RESULT_SIZE: usize = 32 + 1;

    pub const BTC_DECIMALS: usize = 9;
    pub const SOL_DECIMALS: usize = 9;
    pub const MIN_PASSION: u8 = 20;
    pub const MIN_STAMINA: u8 = 20;

    #[program]
    pub mod horse_racing {
        use super::*;

        pub fn initialize(
            ctx: Context<Initialize>,
            operator_list_bump: u8,
            race_bump: u8
        ) -> ProgramResult {

            let operator_list = &mut ctx.accounts.operator_list;
            operator_list.operator_array[0] = ctx.accounts.admin.key();
            operator_list.operator_cnt = 1;
            operator_list.bump = operator_list_bump;

            let race_result = &mut ctx.accounts.race_result;
            race_result.bump = race_bump;

            Ok(())
        }

        pub fn add_operator(
            ctx: Context<AddOperator>
        ) -> ProgramResult {
            let operator_list = &mut ctx.accounts.operator_list;
            let cnt: usize = operator_list.operator_cnt as usize;

            operator_list.operator_array[cnt] = ctx.accounts.operator.key();
            operator_list.operator_cnt += 1;
            Ok(())
        }

        pub fn mint_nft(
            ctx: Context<MintNFT>,
            metadata_bump: u8
        ) -> ProgramResult {

            let upgradable_metadata = &mut ctx.accounts.upgradable_metadata;
            upgradable_metadata.bump = metadata_bump;
            upgradable_metadata.passion = 0;
            upgradable_metadata.stamina = 0;

            let sol_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.sol_feed_account)?;
            let btc_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.btc_feed_account)?;

            //let sol_price: Option<u128> = Some(10);
            //let btc_price: Option<u128> = Some(20);

            msg!("after getprice");
            if let Some(sol_price) = sol_price {
                let rand_from_sol = sol_price + ctx.accounts.clock.unix_timestamp as u128;
                upgradable_metadata.passion = (rand_from_sol % 10) as u8 + MIN_PASSION;
                msg!("Sol Price is {}", sol_price);
            } else {
                upgradable_metadata.passion = (ctx.accounts.clock.unix_timestamp % 10) as u8 + MIN_PASSION;
                msg!("No current Sol price");
            }

            if let Some(btc_price) = btc_price {
                let rand_from_sol = btc_price + ctx.accounts.clock.unix_timestamp as u128;
                upgradable_metadata.stamina = (rand_from_sol % 10) as u8 + MIN_STAMINA;
                msg!("BTC Price is {}", btc_price);
            } else {
                upgradable_metadata.stamina = (ctx.accounts.clock.unix_timestamp % 10) as u8 + MIN_STAMINA;
                msg!("No current BTC price");
            }

            add_nft(
                ctx.accounts.mint.key(), 
                ctx.accounts.nft_list.clone(),
                &upgradable_metadata
            )?;

            Ok(())
        }

        pub fn upgrade_nft(
            ctx: Context<UpgradeNFT>,
            nft_id: u16
        ) -> ProgramResult {

            let upgradable_metadata = &mut ctx.accounts.upgradable_metadata;

            let sol_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.sol_feed_account)?;
            let btc_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.btc_feed_account)?;
            
            //let sol_price: Option<u128> = Some(10);
            //let btc_price: Option<u128> = Some(20);
            
            if let Some(sol_price) = sol_price {
                let rand_from_sol = sol_price + ctx.accounts.clock.unix_timestamp as u128;
                upgradable_metadata.passion = (rand_from_sol % 10) as u8 + upgradable_metadata.passion;
                msg!("Sol Price is {}", sol_price);
            } else {
                upgradable_metadata.passion = (ctx.accounts.clock.unix_timestamp % 10) as u8 + upgradable_metadata.passion;
                msg!("No current Sol price");
            }

            if let Some(btc_price) = btc_price {
                let rand_from_sol = btc_price + ctx.accounts.clock.unix_timestamp as u128;
                upgradable_metadata.stamina = (rand_from_sol % 10) as u8 + upgradable_metadata.stamina;
                msg!("BTC Price is {}", btc_price);
            } else {
                upgradable_metadata.stamina = (ctx.accounts.clock.unix_timestamp % 10) as u8 + upgradable_metadata.stamina;
                msg!("No current BTC price");
            }

            update_nft_list(
                nft_id,
                ctx.accounts.mint.key(), 
                ctx.accounts.nft_list.clone(),
                &upgradable_metadata
            )?;

            sol_transfer(
                ctx.accounts.owner.to_account_info().clone(),
                ctx.accounts.admin.clone(),
                ctx.accounts.system_program.to_account_info().clone(),
                10000000
            )?;
            Ok(())
        }

        pub fn start_race(
            ctx: Context<StartRace>
        ) -> ProgramResult {

            let operator_list = &mut ctx.accounts.operator_list;
            let mut flag: u8 = 0;
            for item in operator_list.operator_array.iter().enumerate() {
                let (_, op): (usize, &Pubkey) = item;
                if *op == ctx.accounts.operator.key() {
                    flag = 1;
                    break;
                }
            }

            if flag != 1 {
                msg!("You are not operator to start race");
                return Err(ProgramError::InvalidAccountData);
            }

            let sol_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.sol_feed_account)?;
            let btc_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.btc_feed_account)?;

            //let sol_price: Option<u128> = Some(10);
            //let btc_price: Option<u128> = Some(20);

            let mut random_value: u128 = 0;
            if let Some(sol_price) = sol_price {
                random_value += sol_price;
                msg!("Sol Price is {}", sol_price);
            } else {
                msg!("No current Sol price");
            }

            if let Some(btc_price) = btc_price {
                random_value += btc_price;
                msg!("BTC Price is {}", btc_price);
            } else {
                msg!("No current BTC price");
            }
            random_value += ctx.accounts.clock.unix_timestamp as u128;

            let nft_count = get_nft_count(ctx.accounts.nft_list.clone())?;
            for i in 0..nft_count {

            }
            let winner_idx = (random_value % nft_count as u128) as u64;
            msg!("winner is {} th horse!", winner_idx);

            // getting Pubkey of winner
            /*let winner_pk = prize_winner((winner_idx % 20) as u8, ctx.accounts.nft_list.clone())?;
            ctx.accounts.race_result.winner = winner_pk;*/
            Ok(())
        }

        
    }

    #[derive(Accounts)]
    #[instruction(operator_list_bump: u8, race_bump: u8)]
    pub struct Initialize<'info> {
        #[account(mut)]
        pub admin: Signer<'info>,

        #[account(
            mut,
            constraint = nft_list_account.owner == program_id
        )]
        pub nft_list_account: AccountInfo<'info>,

        #[account(
            init,
            seeds = [
                b"operator_list".as_ref()
            ],
            bump = operator_list_bump,
            payer = admin,
            space = 8 + OPERATOR_LIST_SIZE
        )]
        pub operator_list: Account<'info, OperatorWhiteList>,

        #[account(
            init,
            seeds = [
                b"race_result".as_ref()
            ],
            bump = race_bump,
            payer = admin,
            space = 8 + RACE_RESULT_SIZE
        )]
        pub race_result: Account<'info, RaceResult>,

        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct AddOperator<'info> {
        #[account(mut, signer)]
        pub admin: AccountInfo<'info>,
        
        pub operator: AccountInfo<'info>,

        #[account(mut)]
        pub operator_list: Account<'info, OperatorWhiteList>,
    }

    #[derive(Accounts)]
    #[instruction(metadata_bump : u8)]
    pub struct MintNFT<'info> {
        #[account(mut, signer)]
        pub admin: AccountInfo<'info>,

        #[account(mut)]
        pub owner: Signer<'info>,

        #[account(
            mut,
            constraint = nft_list.owner == program_id
        )]
        pub nft_list: AccountInfo<'info>,    

        #[account(
            init,
            seeds = [(*mint.key).as_ref()],
            bump = metadata_bump,
            payer = owner,
            space = 8 + UPGRADABLE_METASIZE
        )]
        pub upgradable_metadata: Account<'info, UpgradableMetadata>,

        #[account(owner = spl_token::id())]
        pub mint: AccountInfo<'info>,

        #[account(
            constraint = token_account.mint == *mint.key,
            constraint = *token_account.to_account_info().owner == token::Token::id()
        )]
        pub token_account: Account<'info, TokenAccount>,

        pub sol_feed_account: AccountInfo<'info>,

        pub btc_feed_account: AccountInfo<'info>,

        pub system_program: Program<'info, System>,

        pub clock: Sysvar<'info, Clock>
    }

    #[derive(Accounts)]
    pub struct UpgradeNFT<'info> {
        #[account(
            mut,
            constraint = admin.key() == operator_list.operator_array[0]
        )]
        pub admin: AccountInfo<'info>,

        #[account(mut)]
        pub owner: Signer<'info>,    
        
        #[account(
            mut,
            constraint = nft_list.owner == program_id
        )]
        pub nft_list: AccountInfo<'info>,   

        #[account(
            mut,
            seeds = [(*mint.key).as_ref()],
            bump = upgradable_metadata.bump
        )]
        pub upgradable_metadata: Account<'info, UpgradableMetadata>,

        #[account(owner = spl_token::id())]
        pub mint: AccountInfo<'info>,

        #[account(
            constraint = token_account.mint == *mint.key,
            constraint = *token_account.to_account_info().owner == token::Token::id()
        )]
        pub token_account: Account<'info, TokenAccount>,

        #[account(
            mut,
            seeds = [
                b"operator_list".as_ref()
            ],
            bump = operator_list.bump
        )]
        pub operator_list: Account<'info, OperatorWhiteList>,

        pub sol_feed_account: AccountInfo<'info>,

        pub btc_feed_account: AccountInfo<'info>,

        pub system_program: Program<'info, System>,

        pub clock: Sysvar<'info, Clock>
    }

    #[derive(Accounts)]
    pub struct StartRace<'info> {

        #[account(mut, signer)]
        pub operator: AccountInfo<'info>,

        #[account(mut)]
        pub race_result: Account<'info, RaceResult>,

        #[account(
            mut,
            constraint = nft_list.owner == program_id
        )]
        pub nft_list: AccountInfo<'info>,    
        
        #[account(
            mut,
            seeds = [
                b"operator_list".as_ref()
            ],
            bump = operator_list.bump
        )]
        pub operator_list: Account<'info, OperatorWhiteList>,

        pub sol_feed_account: AccountInfo<'info>,

        pub btc_feed_account: AccountInfo<'info>,

        pub clock: Sysvar<'info, Clock>
    }

    #[account]
    pub struct UpgradableMetadata {
        pub bump: u8,
        pub passion: u8,
        pub stamina: u8,
    }

    #[account]
    pub struct OperatorWhiteList {
        pub operator_array: [Pubkey; 10],
        pub operator_cnt: u8,
        pub bump: u8
    }

    #[account]
    pub struct RaceResult {
        pub winner: Pubkey,
        pub bump: u8
    }

    pub fn add_nft<'info> (
        nft_mint: Pubkey,
        nft_list: AccountInfo<'info>,
        ex_metadata: &UpgradableMetadata
    ) -> ProgramResult {
        let mut count = get_nft_count(nft_list.clone())?;
        let mut nft_list_data = nft_list.data.borrow_mut();

        let item_size: usize = 32 + 1 + 1 + 4;
        let start: usize = (2 + item_size * 32) as usize;
        nft_mint.serialize(&mut &mut nft_list_data[start..start + 32])?;
        nft_list_data[start + 32] = ex_metadata.passion;
        nft_list_data[start + 33] = ex_metadata.stamina;

        let pending_reward: u32 = 0;
        pending_reward.serialize(&mut &mut nft_list_data[start+34..start+38])?;

        count += 1;
        let data = count.to_le_bytes();
        for i in 0..2 {
            nft_list_data[i] = data[i];
        }

        Ok(())
    }
    pub fn update_nft_list<'info> (
        nft_id: u16,
        nft_mint: Pubkey,
        nft_list: AccountInfo<'info>,
        ex_metadata: &UpgradableMetadata
    ) -> ProgramResult {

        let item_size: usize = 32 + 1 + 1 + 4;
        let start: usize = (2 + nft_id * 32) as usize;
        let mut nft_list_data = nft_list.data.borrow_mut();

        let nft_pk_inlist: Pubkey = Pubkey::try_from_slice(&nft_list_data[start..start+32])?;
        if nft_mint != nft_pk_inlist {
            return Err(ProgramError::InvalidAccountData);
        }
        nft_list_data[start + 32] = ex_metadata.passion;
        nft_list_data[start + 33] = ex_metadata.stamina;

        Ok(())
    }

    pub fn prize_winner<'info> (
        random_value: u8,
        nft_list: AccountInfo<'info>
    ) -> ProgramResult {

        let mut nft_list_data = nft_list.data.borrow_mut();
        let count: u16 = u16::try_from_slice(&nft_list_data[0..2])?;

        let item_size: usize = 32 + 1 + 1 + 4;

        let mut t_passion: u8 = 0;
        let mut t_stamina: u8 = 0;

        #[derive(Clone, Copy)]
        struct Score {
            nft_id: u16,
            score: u16
        }
        let mut score_arr: [Score; 1000] = [Score { nft_id: 0, score: 0 }; 1000];

        let cnt: usize = count as usize;

        for i in 0..cnt {
            let start = 2 + item_size * i;
            t_passion = nft_list_data[start + 32];
            t_stamina = nft_list_data[start + 33];
            score_arr[i].score = (t_passion + t_stamina + random_value) as u16;
            score_arr[i].nft_id = i as u16;
        }

        for i in 0..cnt {
            for j in i+1..cnt {
                if score_arr[i].score < score_arr[j].score {
                    let t = score_arr[i];
                    score_arr[i] = score_arr[j];
                    score_arr[j] = t;
                }
            }
        }

        let prize = [0.7, 0.2, 0.03, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01];

        for i in 0..min(cnt as u16, 10) {
            let p: u32 = (prize[i] * 100.0) as u32;
            let idx = score_arr[i].nft_id as usize;
            let start = 2 + item_size*idx;
            p.serialize(&mut &mut nft_list_data[start+34..start+38]);
        }

        Ok(())
    }

    pub fn get_nft_count<'a> (
        nft_list: AccountInfo<'a>
    ) -> Result<u16, ProgramError> {
        let nft_list_data = nft_list.data.borrow();
        let count: u16 = u16::try_from_slice(&nft_list_data[0..2])?;
        Ok(count)
    }


    pub fn get_nft_by_index<'a> (
        idx: u16,
        nft_list: AccountInfo<'a>
    ) -> Result<Pubkey, ProgramError>{
        let nft_list_data = nft_list.data.borrow();
        let item_size: usize = 32 + 1 + 1 + 4;
        let start: usize = (2 + idx as usize * item_size) as usize;
        let winner_pk: Pubkey = Pubkey::try_from_slice(&nft_list_data[start .. start + 32])?;
        Ok(winner_pk)
    }

    // transfer sol
    fn sol_transfer<'a>(
        source: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
        invoke(&ix, &[source, destination, system_program])
    }

    fn min(x: u16, y: u16) -> usize {
        if (x < y) {
            return x as usize;
        }
        y as usize
    }