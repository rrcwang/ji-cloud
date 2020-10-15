use std::collections::HashSet;

pub type Token = String;
pub type Email = String;

#[derive(Clone, Debug)]
pub enum Step {
    Start,
    One,
    Two,
    Three,
    ConfirmEmail,
    Final
}

#[derive(Clone, Debug, Default)]
pub struct RegisterData {
    pub token: Option<String>,
    pub email: Option<String>,
    pub user_name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub edu_resources: bool,
    pub lang: Option<String>,
    pub geocode: Option<String>,
    pub affiliations: HashSet<String>,
    pub age_ranges: HashSet<String>,
    pub subjects: HashSet<String>,
}


#[derive(Debug, Clone)]
pub enum RegisterStatus {
    Busy,
    Failure,
    ConfirmEmail,
    EmptyPw,
    PwMismatch,
    PwWeak,
    EmptyGivenName,
    EmptyLastName,
    EmptyUserName,
    EmptyEmail,
    EmailExists,
    UsernameExists,
    IdExists,
    Geocode,
    Language,
    Over18,
    UnknownFirebase,
    Technical 
}

#[derive(Clone, Copy, Debug)]
pub enum PwInvalid {
    Empty,
    Mismatch
}

impl RegisterStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Busy => "registering...",
            Self::Failure => "failed to register!",
            Self::ConfirmEmail => "confirm your email!",
            Self::EmptyPw => "supply a password!",
            Self::PwMismatch => "passwords don't match!",
            Self::PwWeak => "weak password!",
            Self::EmptyGivenName => "supply a first name!",
            Self::EmptyLastName => "supply a last name!",
            Self::EmptyUserName => "supply a user name!",
            Self::EmptyEmail => "supply an email address!",
            Self::Over18 => "Check the age restriction!",
            Self::Geocode => "Supply a geocode!",
            Self::Language => "Choose a language!",
            Self::UsernameExists => "Username in use!",
            Self::EmailExists => "Email in use!",
            Self::UnknownFirebase => "firebase error!",
            Self::IdExists => "id exists!",
            Self::Technical => "technical error!",
        }.to_string()
    }
}

