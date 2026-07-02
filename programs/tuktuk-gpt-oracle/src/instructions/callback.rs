use anchor_lang::prelude::*;
use solana_gpt_oracle::Identity;

use crate::{state::{Agent, AdoptionScore}};

#[derive(Accounts)]
pub struct CallBack<'info> {

    pub identity: Account<'info, Identity>,

    /// CHECK: Done in code
    pub user: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"AdoptionScore"],
        bump = adoption.bump
    )]
    pub adoption: Account<'info, AdoptionScore>,

}

impl <'info> CallBack <'info> {
    pub fn agent_response(&mut self, response: String) -> Result<()> {

        let score = response.trim().parse::<f32>().unwrap_or(0.0);
        self.adoption.per_score = score;

        Ok(())
    }
}