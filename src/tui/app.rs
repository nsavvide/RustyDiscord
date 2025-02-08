use crate::config::colors::ColorScheme;
use crate::config::colors::Config;
use ratatui::widgets::ListState;
use std::collections::HashMap;

#[derive(Debug)]
pub struct App {
    pub channels: Vec<String>,
    pub private_messages: Vec<String>,
    pub selected_tab: usize,
    pub selected_channel: usize,
    pub list_state: ListState,
    pub messages: HashMap<String, Vec<String>>,
    pub colorschemes: HashMap<String, ColorScheme>,
    pub current_colorscheme: String,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        // Init the colors
        let colorschemes = Config::load().colorschemes;
        let current_colorscheme = "default".to_string();

        Self {
            channels: vec!["general".to_string(), "random".to_string()],
            private_messages: vec!["user1".to_string(), "user2".to_string()],
            selected_tab: 0,
            selected_channel: 0,
            list_state,
            messages: HashMap::new(),
            colorschemes,
            current_colorscheme,
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 2;
        self.list_state.select(Some(0)); // Simply reset the list state
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = if self.selected_tab == 0 { 1 } else { 0 };
        self.list_state.select(Some(0)); // Simply reset the list state
    }

    pub fn next_channel(&mut self) {
        let max = if self.selected_tab == 0 {
            self.channels.len()
        } else {
            self.private_messages.len()
        };
        let selected = self.list_state.selected().unwrap_or(0);
        let new_selected = (selected + 1) % max;
        self.list_state.select(Some(new_selected));
    }

    pub fn previous_channel(&mut self) {
        let max = if self.selected_tab == 0 {
            self.channels.len()
        } else {
            self.private_messages.len()
        };
        let selected = self.list_state.selected().unwrap_or(0);
        let new_selected = if selected == 0 { max - 1 } else { selected - 1 };
        self.list_state.select(Some(new_selected));
    }

    pub fn next_colorscheme(&mut self) {
        let schemes: Vec<&String> = self.colorschemes.keys().collect();
        let current_index = schemes
            .iter()
            .position(|&name| name == &self.current_colorscheme)
            .unwrap_or(0);
        let next_index = (current_index + 1) % schemes.len();
        self.current_colorscheme = schemes[next_index].clone();
    }
}
