use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod smart_contracts {
    use super::*;

    pub fn create_state(
        ctx: Context<CreateState>
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        state.authority = ctx.accounts.authority.key();

        state.post_count = 0;
        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        author_name: String,
        author_url: String,
    ) -> Result {
        let state = &mut ctx.accounts.state;

        let post = &mut ctx.accounts.post;

        post.authority = ctx.accounts.authority.key();

        post.text = text;

        post.author_name = author_name;

        post.author_url = author_url;

        post.comment_count = 0;

        post.index = state.post_count;

        post.post_time = ctx.accounts.clock.unix_timestamp;

        state.post_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateState<'info> {
    #[account(
        init, 
        seeds = [b"state".as_ref()],
        bump,
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    //Authority (this is the signer who paid the transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    // System Program
    pub system_program: UncheckedAccount<'info>,

    // TOken program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,

    // Authenticate Post Account
    #[account(
        init,
        // Post account use "post" and index of post as seed
        seeds = [b"post".as_ref(), state.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + USER_URL_LENGTH + TEXT_LENGTH + USER_NAME_LENGTH
    )]
    pub post: Account<'info, PostAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info, Clock>,

}

#[account]
pub struct StateAccount {
    pub authority: Pubkey,

    pub post_count: u64,
}

//Post Account Structure
#[account]
pub struct PostAccount {
    pub authority: Pubkey,

    pub text: String,

    pub author_name: String,

    pub author_url: String,

    pub comment_count: u64,

    pub index: i64,

    pub post_time: i64,
}