use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("2UA3M6dSJQjrfTdzAiL2vMvubWoch1nTXK5TRaNGuiY4");

#[derive(Accounts)]
#[instruction(id:String)]
pub struct Sample<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Add this account to any instruction Context that needs price data.
    pub price_update: Account<'info, PriceUpdateV2>,
}

#[program]
pub mod pyth_oracle_1 {

    use super::*;

    pub fn sample(ctx: Context<Sample>, id: String) -> Result<()> {
        let price_update = &mut ctx.accounts.price_update;
        let maximum_age: u64 = 30;
        let feed_id: [u8; 32] = get_feed_id_from_hex(&id)?;
        let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

        msg!(
            "The price is ({} ± {}) * 10^{}",
            price.price,
            price.conf,
            price.exponent
        );
        Ok(())
    }
}
