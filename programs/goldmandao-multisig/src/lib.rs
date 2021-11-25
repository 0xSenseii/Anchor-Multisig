use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, Mint};

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
        msg!("INIT DAO");
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintOwnership>) -> ProgramResult {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.dao_token.to_account_info().clone(),
            to: ctx.accounts.member.to_account_info().clone(),
            authority: ctx.accounts.authority.to_account_info().clone(),
        };

        // let seeds = &[ctx.accounts.signer.as_ref(), &[self.nonce]];
        // let signer = &[&seeds[..]];

        let amount = 100;
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        msg!("INIT MINTING");
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
pub struct MintOwnership<'info> {
    pub authority: AccountInfo<'info>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    #[account(mut)]
    pub member: AccountInfo<'info>,
    #[account(mut)]
    dao_token: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct DAOAccount {
    pub dao_name: [u8; 10], // Setting an arbitrary max of ten characters in the ido name.
    pub bumps: PoolBumps,
    pub dao_authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub num_dao_tokens: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct PoolBumps {
    pub dao_account: u8,
    pub redeemable_mint: u8,
}
