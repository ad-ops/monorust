use std::path::PathBuf;

use monorust_models::Checkout;

use crate::services::{git, server};

use super::{
    pages::{checkout, Page},
    Pane,
};

pub struct App {
    pub text: String,
    pub current_page: Page,
    pub current_pane: Pane,
    pub should_quit: bool,
    pub user: String,
    pub module: String,
    pub target_dir: PathBuf,
    pub checkout_message: Option<String>,
    pub checkout_list: Vec<Checkout>,
}
impl App {
    pub fn new(user: &str, module: &str, target_dir: PathBuf) -> Self {
        Self {
            text: String::new(),
            current_page: Page::Help,
            current_pane: Pane::Menu,
            should_quit: false,
            user: user.to_string(),
            module: module.to_string(),
            target_dir,
            checkout_message: None,
            checkout_list: Vec::new(),
        }
    }

    pub fn perform_checkout(&mut self) {
        let git_result = git::checkout(&self.target_dir, &self.module, false);
        let git_message = match git_result {
            Ok(message) => message,
            Err(e) => format!("git checkout encountered error: {e}"),
        };

        let server_checkout = server::checkout_code(&self.user, &self.module, "proj");
        let server_message = match server_checkout {
            Ok(message) => message,
            Err(e) => format!("server checkout encountered error: {e}"),
        };

        let checkout_message = format!("git: {git_message}\nserver: {server_message}");

        self.checkout_message = Some(checkout_message);
    }

    pub fn list_checkouts(&mut self) {
        let checkouts = match server::get_checkouts() {
            Ok(message) => message,
            Err(_e) => Vec::new(),
        };

        self.checkout_list = checkouts;
    }

    pub fn previous_page(&mut self) {
        self.current_page = self.current_page.previous_page();
    }

    pub fn next_page(&mut self) {
        self.current_page = self.current_page.next_page();
    }

    pub fn next_pane(&mut self) {
        self.current_pane = self.current_pane.next_pane();
    }
}
