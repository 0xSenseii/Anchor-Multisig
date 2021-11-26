use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

use std::ops::Deref;

declare_id!("D3ogHpj4ZXexUjWCo2Y1PKZsKVHPY7x4whCCCyq5kHsx");

const DECIMALS: u8 = 6;

#[program]
pub mod goldmandao_multisig {
    use super::*;

    pub fn initialize_dao(
        ctx: Context<InitializeDao>,
        dao_name: String,
        bumps: PoolBumps,
        num_tokens: u64,
    ) -> ProgramResult {
        let dao_account = &mut ctx.accounts.dao_account;

        let name_bytes = dao_name.as_bytes();
        let mut name_data = [b' '; 10];
        name_data[..name_bytes.len()].copy_from_slice(name_bytes);

        dao_account.dao_name = name_data;
        dao_account.bumps = bumps;
        dao_account.dao_authority = ctx.accounts.dao_authority.key();
        dao_account.admins = [
            dao_account.dao_authority,
            dao_account.key(),
            dao_account.key(),
            dao_account.key(),
            dao_account.key(),
        ];
        dao_account.redeemable_mint = ctx.accounts.redeemable_mint.key();
        dao_account.num_dao_tokens = num_tokens;

        Ok(())
    }

    pub fn init_user_redeemable(_ctx: Context<InitUserRedeemable>) -> ProgramResult {
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintOwnership>) -> ProgramResult {
        require!(
            ctx.accounts
                .dao_account
                .admins
                .iter()
                .position(|&a| a == ctx.accounts.user_authority.key())
                .is_some(),
            InvalidAuth
        );
        let dao_name = ctx.accounts.dao_account.dao_name.as_ref();
        let seeds = &[
            dao_name.trim_ascii_whitespace(),
            &[ctx.accounts.dao_account.bumps.dao_account],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.dao_account.to_account_info(),
        };

        let amount = 100;

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_context, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(dao_name: String, bumps: PoolBumps)]
pub struct InitializeDao<'info> {
    #[account(mut)]
    pub dao_authority: Signer<'info>,
    // DAO Account
    #[account(init,
        seeds = [dao_name.as_bytes()],
        bump = bumps.dao_account,
        payer = dao_authority)]
    pub dao_account: Box<Account<'info, DAOAccount>>,

    #[account(init,
        mint::decimals = DECIMALS,
        mint::authority = dao_account,
        seeds = [dao_name.as_bytes(), b"redeemable_mint".as_ref()],
        bump = bumps.redeemable_mint,
        payer = dao_authority)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitUserRedeemable<'info> {
    // User Accounts
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(init,
        token::mint = redeemable_mint,
        token::authority = dao_account,
        seeds = [user_authority.key().as_ref(),
            dao_account.dao_name.as_ref().trim_ascii_whitespace(),
            b"user_redeemable"],
        bump,
        payer = user_authority)]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // DAO Accounts
    #[account(seeds = [dao_account.dao_name.as_ref().trim_ascii_whitespace()],
        bump = dao_account.bumps.dao_account)]
    pub dao_account: Box<Account<'info, DAOAccount>>,
    #[account(seeds = [dao_account.dao_name.as_ref().trim_ascii_whitespace(), b"redeemable_mint"],
        bump = dao_account.bumps.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintOwnership<'info> {
    // User Account
    pub user_authority: Signer<'info>,
    #[account(mut,
        seeds = [
        user_authority.key().as_ref(),
            dao_account.dao_name.as_ref().trim_ascii_whitespace(),
            b"user_redeemable"
        ],
        bump)]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // DAO Accounts
    #[account(seeds = [dao_account.dao_name.as_ref().trim_ascii_whitespace()],
        bump = dao_account.bumps.dao_account)]
    pub dao_account: Box<Account<'info, DAOAccount>>,
    #[account(mut,
        seeds = [dao_account.dao_name.as_ref().trim_ascii_whitespace(), b"redeemable_mint"],
        bump = dao_account.bumps.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct DAOAccount {
    pub dao_name: [u8; 10], // Setting an arbitrary max of ten characters in the ido name.
    pub bumps: PoolBumps,
    pub admins: [Pubkey; 5],
    pub dao_authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub num_dao_tokens: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct PoolBumps {
    pub dao_account: u8,
    pub redeemable_mint: u8,
}

/// Trait to allow trimming ascii whitespace from a &[u8].
pub trait TrimAsciiWhitespace {
    /// Trim ascii whitespace (based on `is_ascii_whitespace()`) from the
    /// start and end of a slice.
    fn trim_ascii_whitespace(&self) -> &[u8];
}

impl<T: Deref<Target = [u8]>> TrimAsciiWhitespace for T {
    fn trim_ascii_whitespace(&self) -> &[u8] {
        let from = match self.iter().position(|x| !x.is_ascii_whitespace()) {
            Some(i) => i,
            None => return &self[0..0],
        };
        let to = self.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
        &self[from..=to]
    }
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid auth token provided")]
    InvalidAuth,
}
