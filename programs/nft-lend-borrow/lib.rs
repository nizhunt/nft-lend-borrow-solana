use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;

pub use states::*;
pub use instructions::*;

declare_id!("4FKLgDkpKwk2s9yhA1U4qN3MV1RRTNUEL35sM5j11onQ");

#[program]
pub mod nft_lend_borrow {
    use super::*;

pub fn create_pool(
        ctx: Context<CreatePool>,
        collection_id: Pubkey,
        duration: i64,
    ) -> Result<()> {
        instructions::create_pool::handler(ctx, collection_id, duration)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
