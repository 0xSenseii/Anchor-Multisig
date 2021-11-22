use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Every method under the #program attribute is an entrypoint to the program that clients may invoke.
#[program]
pub mod anchor_multisig {
    use super::*;
    // Context is a struct containning the program_id with Generic type
    // In this case Initialize is the type of Context
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}


// Accounts on Solana work as files on an OS, they store data of a tx.

/* The account includes metadata for the lifetime of the file. That lifetime 
is expressed by a number of fractional native tokens called lamports.
Accounts are held in validator memory and pay "rent" to stay there. 
Each validator periodically scans all accounts and collects rent. 
Any account that drops to zero lamports is purged. 
Accounts can also be marked rent-exempt if they contain a sufficient number of lamports.
*/

// This macro is a struct of all accounts in our program
#[derive(Accounts)]
pub struct Initialize {}
