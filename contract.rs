use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token};

declare_id!("YOUR_UNIQUE_PROGRAM_ID_HERE");

#[program]
pub mod spl_2022_meme_coin {
    use super::*;
    pub fn create_token(ctx: Context<CreateToken>, total_supply: u64) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let token_account = &mut ctx.accounts.token_account;
        
        token::mint_to(
            ctx.accounts.into_mint_to_context(),
            total_supply
        
        token_account.authority = *mint.to_account_info().key;
        token_account.total_supply = total_supply;
        Ok(())
    }

    // Additional functions for presale and airdrops can be defined here
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct TokenAccount {
    pub authority: Pubkey,
    pub total_supply: u64,
}

impl<'info> CreateToken<'info> {
    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
