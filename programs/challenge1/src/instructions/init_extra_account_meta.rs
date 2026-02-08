use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, 
    state::ExtraAccountMetaList,
    seeds::Seed,
};


#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        ).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![
            // VaultState PDA (derived from mint)
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal { bytes: b"vault_state".to_vec() },
                    Seed::AccountKey { index: 1 }, // mint
                ],
                false,
                false,
            ).unwrap(),

            // Whitelist PDA (derived from vault_state + user)
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal { bytes: b"whitelist".to_vec() },
                    Seed::AccountKey { index: 0 }, // vault_state (resolved above)
                    Seed::AccountKey { index: 3 }, // user / authority
                ],
                false,
                false,
            ).unwrap(),
        ])
    }
}