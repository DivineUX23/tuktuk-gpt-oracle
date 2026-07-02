use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Agent {
    pub context: Pubkey,
    pub bump: u8
}

#[derive(InitSpace)]
#[account]
pub struct AdoptionScore {
    #[max_len(32)]
    pub country: String,
    pub per_score: f32,
    pub bump: u8
}