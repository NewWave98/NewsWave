use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

#[program]
pub mod newswave_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let voting_account = &mut ctx.accounts.voting_account;
        voting_account.authority = ctx.accounts.authority.key();
        voting_account.total_votes = 0;
        voting_account.correct_votes = 0;
        Ok(())
    }

    pub fn submit_vote(
        ctx: Context<SubmitVote>,
        prediction_id: String,
        vote: bool
    ) -> Result<()> {
        let voting_account = &mut ctx.accounts.voting_account;
        let vote_account = &mut ctx.accounts.vote_account;

        // Record the vote
        vote_account.user = ctx.accounts.user.key();
        vote_account.prediction_id = prediction_id;
        vote_account.vote = vote;
        vote_account.timestamp = Clock::get()?.unix_timestamp;

        // Update voting statistics
        voting_account.total_votes += 1;

        Ok(())
    }

    pub fn validate_prediction(
        ctx: Context<ValidatePrediction>,
        prediction_id: String,
        actual_outcome: bool
    ) -> Result<()> {
        let voting_account = &mut ctx.accounts.voting_account;
        let vote_account = &ctx.accounts.vote_account;

        // Verify authority
        require!(
            ctx.accounts.authority.key() == voting_account.authority,
            VotingError::UnauthorizedValidation
        );

        // Update correct votes count if vote matches outcome
        if vote_account.vote == actual_outcome {
            voting_account.correct_votes += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8)]
    pub voting_account: Account<'info, VotingAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitVote<'info> {
    #[account(mut)]
    pub voting_account: Account<'info, VotingAccount>,
    #[account(init, payer = user, space = 8 + 32 + 32 + 1 + 8)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidatePrediction<'info> {
    #[account(mut)]
    pub voting_account: Account<'info, VotingAccount>,
    pub vote_account: Account<'info, VoteAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct VotingAccount {
    pub authority: Pubkey,
    pub total_votes: u64,
    pub correct_votes: u64,
}

#[account]
pub struct VoteAccount {
    pub user: Pubkey,
    pub prediction_id: String,
    pub vote: bool,
    pub timestamp: i64,
}

#[error_code]
pub enum VotingError {
    #[msg("Only the authority can validate predictions")]
    UnauthorizedValidation
}