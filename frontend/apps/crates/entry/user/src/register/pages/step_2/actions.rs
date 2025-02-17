use crate::register::state::{Step, Step2Data};
use super::state::*;
use std::rc::Rc;
use utils::prelude::*;


pub fn submit(state: Rc<State>) {
    state.evaluate_terms_error();
    state.evaluate_language_error();
    state.evaluate_persona_error();

    let terms_error = state.terms_error.lock_ref().is_some();
    let language_error = state.language_error.get();
    let persona_error = state.persona_error.get();


    let persona_error = match &*state.persona.borrow() {
        None => true,
        Some(x) => x.is_empty()
    };
    state.persona_error.set_neq(persona_error);

    let organization_error = match &*state.organization.borrow() {
        None => true,
        Some(x) => x.is_empty()
    };
    state.organization_error.set_neq(organization_error);

    let location_error = match &*state.location_json.borrow() {
        None => true,
        Some(x) => x.is_empty()
    };
    state.location_error.set_neq(location_error);

    if !terms_error 
        && !language_error
        && !persona_error
        && !organization_error
        && !location_error
    {
        next_step(state);
    }

}

impl State {
    pub fn evaluate_terms_error(&self) {
        if !*self.terms.borrow() {
            self.terms_error.set_neq(Some(TermsError::Unchecked));
        } else {
            self.terms_error.set_neq(None);
        }
    }

    pub fn evaluate_language_error(&self) {
        let error = match &*self.language.borrow() {
            None => true,
            Some(x) => x.is_empty()
        };
        self.language_error.set_neq(error);
    }

    pub fn evaluate_persona_error(&self) {
        let error = match &*self.persona.borrow() {
            None => true,
            Some(x) => x.is_empty()
        };
        self.persona_error.set_neq(error);
    }
}

fn next_step(state: Rc<State>) {
    state.step.set(Step::Three(Step2Data{
        step_1: state.step_1.clone(), 
        location_json: state.location_json.borrow().clone(),
        language: state.language.borrow().as_ref().unwrap_ji().clone(), 
        persona: state.persona.borrow().as_ref().unwrap_ji().clone(), 
        organization: state.organization.borrow().as_ref().unwrap_ji().clone(), 
        marketing: state.marketing.borrow().clone(), 
    }));
}
