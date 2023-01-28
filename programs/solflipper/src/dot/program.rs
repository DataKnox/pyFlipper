#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, id, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Flipper {
    pub state: bool,
    pub owner: Pubkey,
}

impl<'info, 'entrypoint> Flipper {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedFlipper<'info, 'entrypoint>> {
        let state = account.state.clone();
        let owner = account.owner.clone();

        Mutable::new(LoadedFlipper {
            __account__: account,
            __programs__: programs_map,
            state,
            owner,
        })
    }

    pub fn store(loaded: Mutable<LoadedFlipper>) {
        let mut loaded = loaded.borrow_mut();
        let state = loaded.state.clone();

        loaded.__account__.state = state;

        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;
    }
}

#[derive(Debug)]
pub struct LoadedFlipper<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Flipper>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub state: bool,
    pub owner: Pubkey,
}

pub fn flip_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut flipper: Mutable<LoadedFlipper<'info, '_>>,
) -> () {
    if !(owner.key() == flipper.borrow().owner) {
        panic!("This is not your calculator!");
    }

    assign!(flipper.borrow_mut().state, !flipper.borrow().state);
}

pub fn initialize_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut flipper: Empty<Mutable<LoadedFlipper<'info, '_>>>,
) -> () {
    let mut flipper = flipper.account.clone();

    assign!(flipper.borrow_mut().owner, owner.key());
}
