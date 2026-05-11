use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, 
    state::ExtraAccountMetaList, seeds::Seed,
};

use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};
use crate::{ID, state::whitelist};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(InitializeExtraAccountMetaList::extra_account_metas()?.len()).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        // let whitelist_account =  vec![ExtraAccountMeta::new_with_seeds(
        //         &[Seed::Literal {
        //             bytes: "whitelist".as_bytes().to_vec(),
                    
        //         },Seed::AccountKey { index: 3 }],
        //         false, // is_signer
        //         false,  // is_writable
        //     )?,];
       
        
        // Ok(whitelist_account)
        Ok(vec![
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: "whitelist".as_bytes().to_vec(),
                    },
                    Seed::AccountKey { index: 3 },
                ],
                false,
                false,
                ).unwrap(),
        ])
    }
}