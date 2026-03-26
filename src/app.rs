use crate::utils::{load_data, save_data};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

fn get_config_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.config/ascii-vault/config.json", home)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AsciiItem {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub db_file: String,
    pub logo_file: String,
}

impl Default for Config {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let base_path = format!("{}/.config/ascii-vault", home);

        Self {
            db_file: format!("{}/library.json", base_path),
            logo_file: format!("{}/logo.txt", base_path),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = get_config_path();
        if Path::new(&path).exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|d| serde_json::from_str(&d).ok())
                .unwrap_or_default()
        } else {
            Config::default()
        }
    }

    pub fn save(&self) {
        let path = get_config_path();
        
        if let Some(p) = Path::new(&path).parent() {
            let _ = fs::create_dir_all(p);
        }

        if let Ok(j) = serde_json::to_string_pretty(self) {
            let tmp_path = format!("{}.tmp", path);
            if fs::write(&tmp_path, &j).is_ok() {
                let _ = fs::rename(&tmp_path, &path);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Browse,
    Edit,
    Rename,
    SetDbPath,
    SetLogoPath,
    ConfirmDelete,
}

pub struct App {
    pub items: Vec<AsciiItem>,
    pub list_state: ListState,
    pub mode: Mode,
    pub edit_buffer: String,
    pub edit_cursor: usize,
    pub edit_scroll: u16,
    pub rename_buffer: String,
    pub path_buffer: String,
    pub status: String,
    pub config: Config,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();
        let items = load_data(&config.db_file);
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            items,
            list_state,
            mode: Mode::Browse,
            edit_buffer: String::new(),
            edit_cursor: 0,
            edit_scroll: 0,
            rename_buffer: String::new(),
            path_buffer: String::new(),
            status: String::new(),
            config,
        }
    }

    pub fn selected(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    pub fn move_item_up(&mut self) {
        let i = self.selected();
        if !self.items.is_empty() && i > 0 {
            self.items.swap(i, i - 1);
            self.list_state.select(Some(i - 1));
            save_data(&self.items, &self.config.db_file);
        }
    }

    pub fn move_item_down(&mut self) {
        let i = self.selected();
        if !self.items.is_empty() && i + 1 < self.items.len() {
            self.items.swap(i, i + 1);
            self.list_state.select(Some(i + 1));
            save_data(&self.items, &self.config.db_file);
        }
    }
}