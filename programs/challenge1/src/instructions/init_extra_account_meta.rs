use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, 
    state::ExtraAccountMetaList,
    seeds::Seed,
};

use crate::ID;


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
        let (vault_config_pda, _bump) = Pubkey::find_program_address(&[b"vault-config"], &ID);
        let vault_config_meta =
            ExtraAccountMeta::new_with_pubkey(&vault_config_pda.to_bytes().into(), false, false).unwrap();
        let user_meta = ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal {
                    bytes: b"user".to_vec(),
                },
                Seed::AccountKey { index: 3 }, // index 3= owner
            ],
            false, // is_signer
            false, // is_writable
        )
        .unwrap();
        let account_metas = vec![user_meta, vault_config_meta];

        Ok(account_metas)
    }
}