use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod LosslessContribution {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, nonce: u8) -> Result<()> {

        let contribution_pool = &mut ctx.accounts.contribution_pool;


    }
    /**
    * Description: This function takes in deposit amount and transfers a deposit into an account
    * Pre: amount > 0
    * Post: Deposit is successfully deposited into correct account
    * Returns: void (Message confirms success)
    * // To Add: Feature to check deposit account is correct and exists
    **/
    pub fn deposit(ctx: Context<Deposit>, amount: u64, nonce: u8> -> Result<()> {
            
        let contributor = &mut ctx.accounts.contributor;

                token::transfer(ctx.accounts.transfer_deposit(), amount)?;

            msg!("Successfully Deposited: {}", ctx.accounts.token_vault.amount);
        Ok(())
    }
    /**
    * Description: This function takes in contribution amount and transfers a contribution into a fundraising account
    * Pre: amount > 0
    * Post: Deposit is successfully deposited into correct fundrasier
    * Returns: void (Message confirms success)
    * // To Add: Feature to check fundraising account is correct and exists
    **/
    pub fn contribute(ctx: Context<contribute>, amount: u64, nonce: u8) -> Result<()> {

        let fundraiser = &mut ctx.accounts.fundraiser;

                token::transfer(ctx.accounts.transfer_contribute(), amount)?;

            msg!("Successfully Contributed: {}", ctx.accounts.token_vault.amount);
        Ok(())

    }

#[derive(Accounts)]
#[instruction(nonce: u8)]
pub struct Initialize {
    #[account(zero)]

    #[account(mut, 
        constraint = token_vault.mint == token_mint.key(),
        constraint = token_vault.owner == signer.key(),
    )]


}

#[derive(Accounts)]
pub struct Deposit {
    #[account(init, payer = contributor, space 8 + 16 + 124)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub token_vault: Account<'info>,
    #[account(mut)]
    pub receiver_token: Account<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program <'info, Token>,
    pub system_program: System <'info, System>,
}

#[derive(Accounts)]
pub struct Contribute {
    #[account(init, payer = user, space 8+8)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub contribution_pool: Box<Account<'info, ContributionPool>>,
    pub fundraiser: Account<'info>,
    pub token_vault: Account<'info>,
    pub sender_token: Account<'info, Token>,
    #[account(mut)]
    pub receiver_token: Account<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program:  Program<'info, System>,
}

impl<'info> Deposit<'info> {
    fn transfer_deposit(&self) -> CpiContext<'_, '_, '_,'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info();
                Transfer {
                    from: self.sender_token.to_account_info(),
                    to: self.receiver_token.to_account_info(),
                    authority: self.contributor.to_account_info(),
                }
        )
    }

    impl<'info> Contribute<'info> {
        fn transfer_contribute(&self) -> CpiContext<'_, '_, '_,'info, Transfer<'info>> {
            CpiContext::new(
                self.token_program.to_account_info();
                    Transfer {
                        from: self.token_vault.to_account_info(),
                        to: self.fundraiser.to_account_info(),
                        authority: self.contributor.to_account_info(),
                    }
            )
        }


#[account]
pub struct Deposit {
    pub contributor: Pubkey,
    pub token_vault: Mint,
    pub token_vault: Pubkey,
    pub receiver_token: Mint,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}

#[account]
pub struct Contribute {
    pub contributor: Pubkey,
    pub fundraiser: Pubkey,
    pub token_vault: Mint,
    pub receiver_token: Mint,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}

#[error]
pub enum ErrorCode {
    #[msg("Insufficient funds to unstake.")]
    InsufficientFundUnstake,
    #[msg("Amount must be greater than zero.")]
    AmountMustBeGreaterThanZero,
}

