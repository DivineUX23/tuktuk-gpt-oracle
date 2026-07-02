use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;

pub use state::*;
pub use instructions::*;

#[error_code]
pub enum ErrorCode {
    #[msg("String too long")]
    StringTooLong,
}

declare_id!("zE1jk5aozH9ndbKvF6LJT4VBUDTGCcRedTiZGwiPs7v");

#[program]
pub mod tuktuk_gpt_oracle {
    use crate::instructions::{Initialize, AgentInput, CallBack};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init(ctx.bumps)
    }

    pub fn agent_input(ctx: Context<AgentInput>, text: String) -> Result<()> {
        ctx.accounts.agent_input(text)
    }

    pub fn agent_response(ctx: Context<CallBack>, text: String) -> Result<()> {
        ctx.accounts.agent_response(text)
    }
}
