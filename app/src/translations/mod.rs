use yew::prelude::*;
use std::rc::Rc;

use core::translations::{Locale, Translation, parse_translations};

fn load_translations(locale: Locale) -> Vec<Translation> {
    match locale {
        Locale::English => parse_translations(include_str!("en.translations")),
        Locale::Swedish => parse_translations(include_str!("sv.translations")),
    }
}

pub fn use_translations() -> Vec<Translation> {
    let translations_state = use_context::<UseReducerHandle<TranslationState>>().expect("no ctx found");

    (*translations_state).translations.clone()
}

pub enum TranslationAction {
    ChangeLocale(Locale),
}

#[derive(PartialEq)]
pub struct TranslationState {
    pub locale: Locale,
    pub translations: Vec<Translation>,
}

impl Default for TranslationState {
    fn default() -> Self {
        let locale = Locale::English;
        let translations = load_translations(locale.clone());

        Self { 
            locale,
            translations,
        }
    }
}

impl Reducible for TranslationState {
    type Action = TranslationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TranslationAction::ChangeLocale(locale) => {
                let translations = load_translations(locale.clone());

                Self {
                    locale,
                    translations,
                }.into()
            },
        }
    }
}