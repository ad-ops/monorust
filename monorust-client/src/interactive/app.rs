use std::{path::PathBuf, fs};

use monorust_models::Checkout;

use crate::services::{git, server};

use super::pages::Page;

pub struct App {
    pub text: String,
    pub current_page: Page,
    pub should_quit: bool,
    pub user: String,
    pub module: String,
    pub target_dir: PathBuf,
    pub checkout_message: Option<String>,
    pub checkout_list: Vec<Checkout>,
    pub removed_dir_message: String,
}
impl App {
    pub fn new(user: &str, module: &str, target_dir: PathBuf) -> Self {
        Self {
            text: String::new(),
            current_page: Page::Help,
            should_quit: false,
            user: user.to_string(),
            module: module.to_string(),
            target_dir,
            checkout_message: None,
            checkout_list: Vec::new(),
            removed_dir_message: String::new(),
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
        self.checkout_list = match server::get_checkouts() {
            Ok(message) => message,
            Err(_e) => Vec::new(),
        };
    }

    pub fn clean_dir(&mut self) {
        self.removed_dir_message = match fs::remove_dir_all(self.target_dir.join("monorust")) {
            Ok(_) => "Directory removed successfully".to_string(),
            Err(e) => format!("Error removing dir: {e}"),
        };
    }

    pub fn previous_page(&mut self) {
        self.current_page = self.current_page.previous_page();
    }

    pub fn next_page(&mut self) {
        self.current_page = self.current_page.next_page();
    }
}
