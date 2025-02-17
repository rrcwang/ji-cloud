use super::state::*;
use components::module::_groups::cards::lookup::Side;
use gloo_timers::future::TimeoutFuture;
use shared::domain::jig::module::body::{_groups::cards::{CardPair, Card}, flashcards::DisplayMode};
use wasm_bindgen_futures::spawn_local;
use crate::base::state::Base;
use std::cell::RefCell;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt}
};
use dominator::clone;
use rand::prelude::*;
use utils::prelude::*;
use crate::base::{
    state::Phase,
    ending::state::Ending
};

impl Game {
    pub fn next(&self) {
        //Cancel flip if it exists
        self.animation_loader.cancel();
        self.gate.set_neq(Gate::Waiting);  

        //borrow-checker fails with if/else here
        {
            if let Some(next) = get_current(&mut self.deck.borrow_mut(), &mut self.rng.borrow_mut()) {
                self.current.set(next);
                return;
            } 
        }

        self.reset_deck();
        //self.base.phase.set(Phase::Ending(Rc::new(Ending::new(self.base.clone()))));
    }

    pub fn flip(state: Rc<Self>) {
        if state.gate.get() == Gate::Waiting {
            state.animation_loader.load(clone!(state => async move {
                state.gate.set(Gate::Flipping);
                TimeoutFuture::new(crate::config::SHOW_TIME).await;
                state.gate.set(Gate::FinishingFlip);  
                TimeoutFuture::new(crate::config::FLIP_TIME).await;
                state.gate.set(Gate::Waiting);  
                state.next();
            }));
        }
    }

    fn reset_deck(&self) {
        let mut rng = self.rng.borrow_mut();

        let mut deck = get_fresh_deck(&self.base, &mut rng); 

        let current = get_current(&mut deck, &mut rng).unwrap_ji();

        *self.deck.borrow_mut() = deck;
        self.current.set(current);
    }

}

pub(super) fn get_fresh_deck(base: &Base, rng:&mut ThreadRng) -> Vec<CardPair> {
    let mut deck = base.raw_pairs.clone();
    
    deck.shuffle(rng); 

    deck
}

pub(super) fn get_current(deck: &mut Vec<CardPair>, rng: &mut ThreadRng) -> Option<Current> {
    deck.pop()
        .map(|pair| {
   
            if rng.gen::<bool>() { 
                Current {
                    card: pair.0,
                    other: pair.1,
                    side: Side::Left 
                }
            } else { 
                Current {
                    card: pair.1,
                    other: pair.0,
                    side: Side::Right
                }
            }
        })
}
