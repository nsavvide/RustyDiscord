use std::collections::HashMap;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct App {
    pub channels: Vec<String>,                  // List of channels
    pub private_messages: Vec<String>,          // List of private messages
    pub selected_tab: usize,                    // Selected tab (0 = Channels, 1 = PMs)
    pub selected_channel: usize,                // Selected channel or PM index
    pub list_state: ListState,                  // State of the list widget
    pub messages: HashMap<String, Vec<String>>, // Messages for each channel/PM
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            channels: vec!["general".to_string(), "random".to_string()],
            private_messages: vec!["user1".to_string(), "user2".to_string()],
            selected_tab: 0,
            selected_channel: 0,
            list_state,
            messages: HashMap::new(),
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
        let new_selected = if selected == 0 {
            max - 1
        } else {
            selected - 1
        };
        self.list_state.select(Some(new_selected));
    }
}
