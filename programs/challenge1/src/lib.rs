use anchor_lang::prelude::*;

declare_id!("7cwdqRZ1Ap8ano7Vsdwk9NfkB26tWf8bSba3Bvb2G6JM");

#[program]
pub mod challenge1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
