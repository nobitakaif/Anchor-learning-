use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }

    pub fn initialize(ctx : Context<Initialize>)-> Result<()>{
        ctx.accounts.calculator.result =0;
        ctx.accounts.calculator.payer = ctx.accounts.payer.key();
        print!("your current state of the result -> {:?}", ctx.accounts.calculator.result);
        print!("who is the singer of your account -> {:?}", ctx.accounts.calculator.payer);
        msg!("current state of result {:?}", ctx.accounts.calculator.result);
        msg!("who is signer {:?}", ctx.accounts.calculator.payer);
        Ok(())
    }

    pub fn add(ctx:Context<Add>, a:u8, b:u8)->Result<()>{
        ctx.accounts.add.result = a+b;
        msg!("Success");
        Ok(())
    }
}

// this is similar as class which we're defining how it look like, ths is eventually stored in blockchain
#[account]
pub struct CalculatorTemplate{
    result : u8,
    payer : Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // singer using #[account(mut)]
    #[account(mut)]
    payer : Signer<'info>, // payer -> who is going to pay for this account 

    // for actually creating the object 
    #[account(
        init,
        space=8+1+32, // 8 should be always, 1 bytes for result u8(8 bits== 1 bytes), 32 for pub key(for the key always 32 bytes)
        payer = payer // who will pay for this account
    )]
    calculator : Account<'info, CalculatorTemplate>,
    system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Add<'info>{
    #[account(mut)]
    add : Account<'info, CalculatorTemplate>
}