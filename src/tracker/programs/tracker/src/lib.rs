use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("rS9PUxa2aE2XmEMVXSkx6owFpGyiVjpBfYCVMD5Yy9v");

#[program]
pub mod tracker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user = &mut ctx.accounts.user;
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn verify_time(
        ctx: Context<TransferLamports>,
        amount: u64,
        time_in_secs: u64,
    ) -> Result<()> {
        if time_in_secs < 3600 {
            let from_account = &ctx.accounts.from;
            let to_account = &ctx.accounts.to;

            let transfer_instruction =
                system_instruction::transfer(from_account.key, to_account.key, amount);

            anchor_lang::solana_program::program::invoke_signed(
                &transfer_instruction,
                &[
                    from_account.to_account_info(),
                    to_account.clone(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[],
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8, seeds = [b"vault_1", user.key.as_ref()], bump)]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultAccount {}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"vault_1", user.key.as_ref()], bump)]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK: This can be any account; we don't perform any data validation here.
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
