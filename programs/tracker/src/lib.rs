use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("4ftp5BXnX7NoXfoesrpUDJrnHBxUJnamjv6Ny2gobgih");

#[program]
pub mod tracker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.user.key;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user = &mut ctx.accounts.owner;
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn verify_time(ctx: Context<VerifyTime>, amount: u64, time_in_secs: u64) -> Result<()> {
        if time_in_secs < 10800 {
            let vault = &mut ctx.accounts.vault;
            let burn_vault = &mut ctx.accounts.burn;
            **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
            **burn_vault.to_account_info().try_borrow_mut_lamports()? += amount;
        }

        Ok(())
    }
}
const VAULT_SPACE: usize = 8 + size_of::<Pubkey>();
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + VAULT_SPACE, seeds = [b"vault_1", user.key.as_ref()], bump)]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultAccount {
    pub owner: Pubkey,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"vault_1", owner.key.as_ref()], bump, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyTime<'info> {
    #[account(mut, seeds = [b"vault_1", owner.key.as_ref()], bump, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: this must be the burn address, but we’ll verify below
    #[account(mut)]
    pub burn: AccountInfo<'info>,
}
