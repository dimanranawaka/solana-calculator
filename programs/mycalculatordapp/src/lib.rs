use anchor_lang::prelude::*;

declare_id!("HjpXgiRpxr3iJP9AeRxr6xzyqMMEUH5BzbfkUr8j1mTX");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Create>,init_message:String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(());
    }
}

#[derive(Accounts)]
pub struct Initialize {}
