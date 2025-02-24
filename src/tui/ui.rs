use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame
};

use ratatui::prelude::*;

use crate::tui::app::App;

pub fn render(f: &mut Frame, app: &App) {
    // Split the layout into three parts
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80), // Chat area
                Constraint::Percentage(20), // Input bar
            ]
            .as_ref(),
        )
        .split(f.area());

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20), // Sidebar
                Constraint::Percentage(80), // Chat area
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    // Render the sidebar
    render_sidebar(f, app, inner_chunks[0]);

    // Render the chat area
    render_chat_area(f, app, inner_chunks[1]);

    // Render the input bar
    render_input_bar(f, app, chunks[1]);
}

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let colorscheme = &app.colorschemes[&app.current_colorscheme];

    // Split the sidebar area into two parts: one for the tabs and one for the list
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    let tabs = Tabs::new(vec!["Channels", "Private Messages"])
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.selected_tab)
        .style(Style::default().fg(get_color(&colorscheme.text)))
        .highlight_style(Style::default().fg(get_color(&colorscheme.highlight)).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);

    let list_items: Vec<ListItem> = if app.selected_tab == 0 {
        app.channels
            .iter()
            .map(|channel| ListItem::new(Span::raw(channel)))
            .collect()
    } else {
        app.private_messages
            .iter()
            .map(|pm| ListItem::new(Span::raw(pm)))
            .collect()
    };

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(Style::default().fg(get_color(&colorscheme.highlight)).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, sidebar_chunks[1], &mut app.list_state.clone());
}

fn render_chat_area(f: &mut Frame, app: &App, area: Rect) {
    let colorscheme = &app.colorschemes[&app.current_colorscheme];

    let messages = app
        .messages
        .get(&app.channels[app.selected_channel])
        .map(|msgs| msgs.as_slice())
        .unwrap_or(&[]);

    let text: Vec<Line> = messages
        .iter()
        .map(|msg| Line::from(Span::styled(msg, Style::default().fg(get_color(&colorscheme.text)))))
        .collect();

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Chat"))
        .scroll((messages.len() as u16, 0));

    f.render_widget(paragraph, area);
}

fn render_input_bar(f: &mut Frame, app: &App, area: Rect) {
    let colorscheme = &app.colorschemes[&app.current_colorscheme];

    let input = Paragraph::new("Type your message here...")
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default().fg(get_color(&colorscheme.text)));

    f.render_widget(input, area);
}

fn get_color(color_name: &str) -> Color {
    match color_name {
        "white" => Color::White,
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "gray" => Color::Gray,
        _ => Color::White, // Default color
    }
}
