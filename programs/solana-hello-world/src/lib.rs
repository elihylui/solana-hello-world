use anchor_lang::prelude::*;

declare_id!("4hsocDfvofGq5CgSvj8zVVn5Bz3ExycABQKVQNZYeMjs"); 
//this is the public key i.e. "program-id" of the smart contract itself - should be the same as the "solana_hello_world" under [programs.devnet] in `Anchor.toml`
//get this after the smart contract is being deployed

#[program]
pub mod solana_hello_world {
    use super::*;

    pub fn create_message(ctx: Context<CreateMessage>, content: String) -> Result<()> {
        let message: &mut Account<Message> = &mut ctx.accounts.message;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        message.author = *author.key;
        message.timestamp = clock.unix_timestamp;
        message.content = content;
        
        Ok(())
      }

      pub fn update_message(ctx: Context<UpdateMessage>, content: String) -> Result<()> {
        let message: &mut Account<Message> = &mut ctx.accounts.message;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        message.author = *author.key;
        message.timestamp = clock.unix_timestamp;
        message.content = content;
        
        Ok(())
      }
}

#[account]
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}

#[derive(Accounts)]
pub struct CreateMessage<'info> {
        #[account(init, payer = author, space = 1000)]
    pub message: Account<'info, Message>,
        #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
        #[account(mut)]
    pub message: Account<'info, Message>,
        #[account(mut)]
    pub author: Signer<'info>,
}
