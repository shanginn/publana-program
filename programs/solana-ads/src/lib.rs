use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};
use anchor_lang::solana_program::{
    system_instruction,
    system_program,
    program::invoke,
    native_token::lamports_to_sol,
};

use std::mem::size_of;

declare_id!("GMyP4dhUir2sc41zTGRavav79nfim1WaTp2FUmiThHU2");

#[error]
pub enum ErrorCode {
    #[msg("The provided title should be 280 characters long maximum.")]
    TitleTooLong,

    #[msg("Can not update Ad. Text limit will be exceeded.")]
    TextLimitExceeded,
}

const CREATORS_KEYS: [Pubkey; 2] = [
    // hobB4CWoWbxatFFXfCJTKEeoWXbFfTDbrQmRdGr7aUz
    Pubkey::new_from_array([
        10u8,  115u8, 233u8, 51u8,  5u8,   207u8, 142u8, 21u8,
        147u8, 47u8,  224u8, 44u8,  100u8, 226u8, 179u8, 253u8,
        113u8, 232u8, 83u8,  245u8, 34u8,  17u8,  223u8, 237u8,
        29u8,  185u8, 74u8,  230u8, 0u8,   72u8,  12u8,  91u8
    ]),

    // B27hkJAvhk8bb2x7ytyNoXSRy9CvEQ5sWyTqGGMZghEF
    Pubkey::new_from_array([
        148u8, 222u8, 7u8,   85u8,  28u8,  77u8,  51u8,  150u8,
        49u8,  109u8, 4u8,   45u8,  57u8,  197u8, 87u8,  61u8,
        177u8, 218u8, 67u8,  51u8,  111u8, 121u8, 105u8, 152u8,
        192u8, 188u8, 199u8, 160u8, 36u8,  167u8, 22u8,  120u8
    ])
];

const MAX_TITLE_LENGTH: usize = 280;
const BASE_TAX_LAMPORTS: u64 = 100000;
const TAX_PER_RANK_LAMPORTS: u64 = 10000;

enum Categories {
    Buy,
    Sell,
    LookingForJob,
    JobOffer,
    NFT,
    Other,
}

#[program]
pub mod solana_ads {
    use super::*;

    pub fn create_ad(
        ctx: Context<CreateAd>,
        title: String,
        content: String,
        image: String,
        text_limit: u32,
        rank: u64,
    ) -> ProgramResult {
        let ad = &mut ctx.accounts.ad;
        let authority = &mut ctx.accounts.authority;
        let clock: Clock = Clock::get().unwrap();

        msg!("Begin creating ad");

        if title.chars().count() > MAX_TITLE_LENGTH {
            return Err(ErrorCode::TitleTooLong.into());
        }

        ad.title = title;
        ad.content = content;
        ad.image = image;
        ad.authority = authority.key();
        ad.timestamp = clock.unix_timestamp;
        ad.text_limit = text_limit;
        ad.rank = rank;

        let tax = BASE_TAX_LAMPORTS + TAX_PER_RANK_LAMPORTS * rank;

        msg!("Tax: {} SOL", lamports_to_sol(tax));

        transfer_tax(ctx, tax).unwrap();

        Ok(())
    }

    pub fn transfer_tax(ctx: Context<CreateAd>, tax: u64) -> ProgramResult {
        let authority = &mut ctx.accounts.authority;
        let kolyan_account = &ctx.accounts.kolyan_account;
        let viktrch_account = &ctx.accounts.viktrch_account;
        let system_program = &ctx.accounts.system_program;
        let derived_address = &ctx.accounts.derived_address;

        invoke(
            &system_instruction::transfer(&authority.key(), &ctx.accounts.derived_address.key(), tax),
            &[authority.to_account_info(), derived_address.to_account_info()],
        )?;

        invoke(
            &system_instruction::transfer_with_seed(
                &derived_address.key(),
                &authority.key(),
                "seed".to_string(),
                &ctx.program_id,
                &viktrch_account.key(),
                tax / 2
            ),
            &[
                authority.to_account_info(),
                viktrch_account.to_account_info(),
                system_program.to_account_info(),
                derived_address.to_account_info(),
            ],
        )?;

        msg!("Half sent to Victrch");

        invoke(
            &system_instruction::transfer_with_seed(
                &derived_address.key(),
                &authority.key(),
                "seed".to_string(),
                &ctx.program_id,
                &kolyan_account.key(),
                tax / 2
            ),
            &[
                authority.to_account_info(),
                kolyan_account.to_account_info(),
                system_program.to_account_info(),
                derived_address.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn update_ad(ctx: Context<UpdateAd>, title: String, content: String) -> ProgramResult {
        let ad = &mut ctx.accounts.ad;

        if title.chars().count() as u32 + content.chars().count() as u32 > ad.text_limit {
            return Err(ErrorCode::TextLimitExceeded.into());
        }

        ad.title = title;
        ad.content = content;

        Ok(())
    }

    pub fn append_ad_content(ctx: Context<AppendAdContent>, content: String) -> ProgramResult {
        let ad = &mut ctx.accounts.ad;

        if ad.title.chars().count() as u32
            + ad.content.chars().count() as u32
            + content.chars().count() as u32
            > ad.text_limit
        {
            return Err(ErrorCode::TextLimitExceeded.into());
        }

        ad.content += &content;

        Ok(())
    }

    pub fn delete_ad(ctx: Context<DeleteAd>) -> ProgramResult {
        let _ad = &mut ctx.accounts.ad;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAd<'info> {
    #[account(init, payer = authority, space = Ad::size(ix_data))]
    pub ad: Account<'info, Ad>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub derived_address: AccountInfo<'info>,

    #[account(address = CREATORS_KEYS[0], mut)]
    pub kolyan_account: UncheckedAccount<'info>,

    #[account(address = CREATORS_KEYS[1], mut)]
    pub viktrch_account: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateAd<'info> {
    #[account(mut, has_one = authority)]
    pub ad: Account<'info, Ad>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteAd<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority,
    )]
    pub ad: Account<'info, Ad>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AppendAdContent<'info> {
    #[account(mut, has_one = authority)]
    pub ad: Account<'info, Ad>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Ad {
    pub title: String,
    pub content: String,
    pub image: String,
    pub timestamp: i64,
    pub authority: Pubkey,
    pub text_limit: u32,
    pub rank: u64
}

const DISCRIMINATOR_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;

impl Ad {
    pub fn size(ix_data: &[u8]) -> usize {
        msg!("Calculating account size");

        #[derive(BorshDeserialize, BorshSerialize)]
        struct CreateAdArgs {
            pub title: String,
            pub content: String,
            pub image: String,
            pub text_len: u32,
            pub rank: u64,
        }

        let create_ad_args: CreateAdArgs = CreateAdArgs::try_from_slice(&ix_data[..]).unwrap();

        let size = DISCRIMINATOR_LENGTH
            + STRING_LENGTH_PREFIX
            + STRING_LENGTH_PREFIX
            + STRING_LENGTH_PREFIX
            + (create_ad_args.text_len as usize) * 4
            + create_ad_args.image.chars() * 4
            + size_of::<i64>() // Timestamp
            + size_of::<u64>() // Rank
            + size_of::<Pubkey>() // Public key
            + size_of::<u32>(); // Text length value

        msg!("Account size: {}", size);

        size
    }
}