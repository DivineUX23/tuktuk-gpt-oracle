use anchor_lang::prelude::*;
use solana_gpt_oracle::cpi::accounts::CreateLlmContext;
use solana_gpt_oracle::cpi::create_llm_context;
use solana_gpt_oracle::{ContextAccount, Counter, Identity};

use crate::{state::{Agent, AdoptionScore}};

const SYSTEM_PROMPT: &str = "Act as a macroeconomic data oracle. Your task is to calculate the precise mathematical probability (0.00% to 100.00%) of B2B/B2C merchant businesses in the specified country adopting cryptocurrency as a standard payment rail.

### Input Variables to Evaluate:
1. Population dynamics and urbanization density.
2. Digital Infrastructure: Mobile internet penetration, smartphone availability, and hardware access.
3. Economic Metrics: GDP velocity, transaction volumes, and informal market size.
4. Socio-Economic/Buying Power: Disposable income, inflation rates, and local currency devaluation risk.
5. Crypto Penetration: P2P transaction volume, historical retail adoption index, and regulatory stance.

### Computation Protocol:
Evaluate the systemic incentives (inflation hedging, payment transaction fee arbitrage, unbanked liquidity gaps) against the frictions (regulatory penalties, volatility risk, infrastructure limits). Synthesize these vectors into a final percentage probability.

### Strict Output Constraints:
You must output ONLY raw numbers representing the calculated probability rounded to two decimal places. 
- Do NOT include the \"%\" sign.
- Do NOT include letters, words, spaces, headers, punctuation, or any unique characters.
- Output exactly 4 to 5 digits representing the percentage (e.g., if the probability is 64.21%, your output must be exactly: 64.21).
";


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Agent::INIT_SPACE,
        seeds = [b"agent"],
        bump
    )]
    pub agent: Account<'info, Agent>,

    #[account(mut)]
    pub llm_context: Account<'info, ContextAccount>,

    #[account(mut)]
    pub counter: Account<'info, Counter>,

    #[account(address = solana_gpt_oracle::ID)]
    /// CHECK: Done in code
    pub oracle_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

}

impl <'info> Initialize <'info> {
    pub fn init(&mut self, bumps: InitializeBumps) -> Result<()> {

        self.agent.context = self.llm_context.key();

        let accounts = CreateLlmContext {
            payer: self.signer.to_account_info(),
            counter: self.counter.to_account_info(),
            context_account: self.llm_context.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        let cpi_program = self.oracle_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, accounts);

        create_llm_context(cpi_ctx, SYSTEM_PROMPT.to_string())?;

        Ok(())

    }
}