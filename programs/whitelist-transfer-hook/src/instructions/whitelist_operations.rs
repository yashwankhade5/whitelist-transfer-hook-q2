use anchor_lang::{
    prelude::*, 
    system_program
};
use anchor_spl::token_interface::{
    Mint, 
    TokenInterface,
};

use crate::state::whitelist::Whitelist;

#[derive(Accounts)]
pub struct AddWhitelistOperation<'info> {
    #[account(
        mut,
        address = mint.mint_authority.unwrap()
    )]
    pub admin: Signer<'info>,
    #[account(init,
        payer=admin,
        space=8+4+1,
        seeds = [b"whitelist",user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK:  not needed
    pub user:SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddWhitelistOperation<'info> {
    pub fn add_to_whitelist(&mut self) -> Result<()> {
        self.whitelist.set_inner(Whitelist { bump: self.whitelist.bump });
    
        Ok(())
    }

}

#[derive(Accounts)]
pub struct RemoveWhitelistOperation<'info> {
    #[account(
        mut,
       address = mint.mint_authority.unwrap()
    )]
    pub admin: Signer<'info>,
    #[account(mut,
        close=admin,
        seeds = [b"whitelist",user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
     pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK:not needed
    pub user:UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveWhitelistOperation<'info> {
    pub fn remove_to_whitelist(&mut self) -> Result<()> {
        
        Ok(())
    }

}