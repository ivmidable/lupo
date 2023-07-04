use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{state::{config::DaoConfig, Proposal, StakeState, ProposalType}, errors::DaoError};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreatePrediction<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds=[b"stake", config.key().as_ref(), owner.key().as_ref()],
        bump = stake_state.state_bump
    )]
    stake_state: Account<'info, StakeState>,
    #[account(
        init,
        payer = owner,
        seeds=[b"proposal", config.key().as_ref(), id.to_le_bytes().as_ref()],
        bump,
        space = Prediction::LEN
    )]
    prediction: Account<'info, Prediction>,
    #[account(
        seeds=[b"treasury", config.key().as_ref()],
        bump = config.treasury_bump
    )]
    treasury: SystemAccount<'info>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    system_program: Program<'info, System>
}

impl<'info> CreatePrediction<'info> {

    pub fn create_prediction(
        &mut self,
        game_id: u64,
        prediction: PredictionType,
        store_prediction: u64,
        bump: u8
    ) -> Result<()> {
        // Make sure user has staked
        self.stake_state.check_stake()?;
        // Check ID and add prediction
        self.config.add_prediction(game_id)?;
        // Initialize the prediction
        self.prediction.init(
            game_id,
            prediction,
            store_prediction,
            bump
        )
    }

    pub fn pay_prediction_fee(
        &mut self
    ) -> Result<()> {
        let accounts = Transfer {
            from: self.owner.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let ctx = CpiContext::new(
            self.system_program.to_account_info(),
            accounts
        );

        transfer(ctx, self.config.prediction_fee)
    }
}