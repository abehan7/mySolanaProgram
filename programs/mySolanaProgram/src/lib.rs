use anchor_lang::prelude::*;
// import library


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// The declareId is where Solana stores the address or program id of your program.
// By default, Anchor generates a program id for us.


#[program]
// Then, you will see a #[program] section. 
// This is the program module and is where the logic of the program lives.
pub mod my_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

// Finally, there is a #[derive(Acccounts)] section. 
// This is where the Accounts struct lives which is where accounts are validated.
// struct is interface for the program. like ts
#[derive(Accounts)]
pub struct Initialize {}
// 원래 ethereum에서는 밖에 있는 contract참조하려면 address넣고 뭐 하고 해야했는데
// 여기는 참조키? 같은게 있데. 그게 편리한 점인 듯



