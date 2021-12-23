use anchor_lang::prelude::*;

declare_id!("8Jg459KK5YRahoEkBBzNLU3HMNrqD1DuauUff3apJpou");

#[program]
pub mod messengerapp {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult{
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user,  space = 64 + 64 + 32 + 32)]
    pub base_account: Account<'info, Messenger>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Update<'info>{
    #[account(mut)]
    pub base_account: Account<'info, Messenger>,

}
#[account]
pub struct Messenger{
    pub to: Pubkey, //for added functionality in the future
    pub from: Pubkey,//for added functionality inthe future
    pub data: String,
    pub data_list: Vec<String>,
    
}
}