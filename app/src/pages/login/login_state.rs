use yew::prelude::*;
use std::rc::Rc;
use gloo::storage::{SessionStorage, Storage};

pub enum LoginAction {
    Login(String),
    Logout,
}

#[derive(PartialEq)]
pub struct LoginState {
    pub logged_in: bool,
}

impl Default for LoginState {
    fn default() -> Self {
        let logged_in: bool = SessionStorage::get("test").unwrap_or_default();

        Self { logged_in }
    }
}

impl Reducible for LoginState {
    type Action = LoginAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let logged_in = match action {
            LoginAction::Login(key) => login(key),
            LoginAction::Logout => {
                logout();
                false
            },
        };

        Self { logged_in }.into()
    }
}

fn login(key: String) -> bool {
    if key == "asd" {
        let _ = SessionStorage::set("test", true);
        return true;
    }

    SessionStorage::delete("test");
    false
}

fn logout() {
    SessionStorage::delete("test");
}