use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock::Clock, sysvar::Sysvar, system_instruction};

declare_id!("EB23s1ATNmzikboXtK7QWwvZjFEd2wdB34X4tu6p7B5m");

#[program]
pub mod auction_dapp {
    use super::*;

    pub fn start(ctx: Context<StartAuction>, name: String, name_of_item: String, min_bid: u8, duration: u8) -> Result<()> {
        require!(name.len() <= Auction::MAX_STR_LEN, Errors::MaxStrLenExceeded);
        require!(name_of_item.len() <= Auction::MAX_STR_LEN, Errors::MaxStrLenExceeded);

        msg!("Starting Auction.....");
        let auction = &mut ctx.accounts.auction;
        let clock = Clock::get().unwrap();
        let timestamp = clock.unix_timestamp;

        auction.creator = ctx.accounts.creator.key();
        auction.name = name;
        auction.duration = duration;
        auction.started_at = timestamp;
        auction.has_ended = false;
        auction.name_of_item = name_of_item;
        auction.last_bid = min_bid;
        auction.winner = None;

        msg!("Auction has started.");

        Ok(())
    }

    pub fn bid(ctx: Context<Bidding>, name: String, amount: u8) -> Result<()> {
        msg!("Bidding for {} at {} by {} is initialized", name, amount, ctx.accounts.bidder.key());
        let auction = &mut ctx.accounts.auction;

        require!(!auction.has_ended, Errors::HasClosed);
        require!(amount > auction.last_bid, Errors::InsufficentBid);

        auction.last_bid = amount;
        auction.winner = Some(ctx.accounts.bidder.key());

        msg!("Bidding for {} at {} by {} is approved", name, amount, ctx.accounts.bidder.key());

        Ok(())
    }

    pub fn close(ctx: Context<CloseAuction>, name: String) -> Result<()> {
        msg!("Closing auction for {}", name);
        let auction = &mut ctx.accounts.auction;
        let clock = Clock::get().unwrap();
        let timestamp = clock.unix_timestamp;
        let duration = (timestamp - auction.started_at) / 60;

        require!(duration >= auction.duration.into(), Errors::HasNotClosed);

        auction.has_ended = true;

        let winner = match auction.winner {
            Some(key) => key,
            None => panic!("A fatal error just occured.")
        };
        msg!("Closed auction for {} and the winner is {}", name, winner);

        let lamports_transfer_instruction = system_instruction::transfer(&winner, &auction.creator, auction.last_bid.into());
        anchor_lang::solana_program::program::invoke_signed(
            &lamports_transfer_instruction,
            &[ctx.accounts.winner.to_account_info(), ctx.accounts.creator.clone(), ctx.accounts.system_program.to_account_info()],
            &[],
        ).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, name_of_item: String)]
pub struct StartAuction<'info> {
    #[account(init, seeds = [name.as_bytes()], bump, payer = creator, space = Auction::INIT_SPACE + name.len() + name_of_item.len())]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Bidding<'info> {
    #[account(mut, seeds = [name.as_bytes()], bump)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CloseAuction<'info> {
    #[account(mut, seeds = [name.as_bytes()], bump, close = creator)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub winner: Signer<'info>,
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Auction {
    creator: Pubkey,
    name: String,
    duration: u8,
    started_at: i64,
    has_ended: bool,
    name_of_item: String,
    last_bid: u8,
    winner: Option<Pubkey>,
}

impl Space for Auction {
    const INIT_SPACE: usize = 8 + 32 + 4 + 1 + 8 + 1 + 4 + 1 + (1 + 32);
}

impl Auction {
    const MAX_STR_LEN: usize = 10;
}

#[error_code]
pub enum Errors {
    #[msg("The current bid must exceed the previous bid")]
    InsufficentBid,

    #[msg("String is too long")]
    MaxStrLenExceeded,

    #[msg("Auction has closed")]
    HasClosed,

    #[msg("Auction has not closed")]
    HasNotClosed,

    #[msg("Cannot perform this operation")]
    NotCreator,
}