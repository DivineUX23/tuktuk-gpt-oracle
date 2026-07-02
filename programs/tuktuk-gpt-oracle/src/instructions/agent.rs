use anchor_lang::prelude::*;
use solana_gpt_oracle::{InteractWithLlm, cpi::{
    accounts::{CreateLlmContext, interactWithLlm},
    create_llm_context, interact_with_llm,
}};
use solana_gpt_oracle::{ContextAccount, Counter, Identity};

use crate::{instructions::callback, state::{AdoptionScore, Agent}};

#[derive(Accounts)]
pub struct AgentInput<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = AdoptionScore::DISCRIMINATOR.len() + AdoptionScore.INIT_SPACE.len(),
        seeds = [b"AdoptionScore"],
        bump
    )]
    pub adoption: Account<'info, AdoptionScore>,

    #[account(mut)]
    /// CHECK: Done in code
    pub interaction: AccountInfo<'info>,

    #[account(seeds = [b"agent"], bump = agent.bump)]
    pub agent: Account<'info, Agent>,

    #[account(address = agent.context)]
    pub context_account: Account<'info, ContextAccount>,

    #[account(address = solana_gpt_oracle::ID)]
    pub oracle_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

}

impl <'info> AgentInput <'info> {
    pub fn agent_input(&mut self, text: String) -> Result<()> {

        require!(text.len() <= 32, ErrorCode::StringTooLong);

        self.adoption.country = text;

        let accounts = InteractWithLlm {
            payer: self.user,
            interaction: self.interaction,
            context_account: self.context_account,
            system_program: self.system_program,
        };

        let cpi_program = self.oracle_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, accounts)?;

        let callback_disc = callback::DISCRIMINATOR.try_into().expect("must be 8 bytes");
        //create_llm_context(cpi_ctx, SYSTEM_PROMPT.to_string())?;

        let account_metas = (
            solana_gpt_oracle::AccountMeta {
                pubkey: self.user.to_account_info().key(),
                is_signer: true,
                is_writable: false
            },
            solana_gpt_oracle::AccountMeta {
                pubkey: self.adoption.to_account_info().key(),
                is_signer: true,
                is_writable: false
            }
        );

        interact_with_llm(
            cpi_ctx, 
            text, 
            crate::ID, 
            callback_disc, 
            Some(vec![account_metas])
        )?;

        Ok(())

    }
}