use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
    
    let title = Paragraph::new(Text::styled(
        "Create new Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);


    // Key-value pair list
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }
    let list = List::new(list_items);
    f.render_widget(list, chunks[1]);


    // Bottom info-bar
    let current_navigation_text = vec![
        // the first half of text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green))
            CurrentScreen::Editing => {
                Span::styled("Editing Json key", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting!", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),

        // divider bar to separate 2 sections
        Span::styled(" | ", Style::default().fg(Color::White)),

        // the final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Json key", Style::default().fg(Color::Green))
                    },
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Json value", Style::default().fg(Color::LightGray))
                    },
                }
            } else {
                Span::styled("Not currently editing...", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair", Style::default().fg(Color::Red)
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel / (Tab) to switch boxes / (ENTER) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint)).bloc(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);


    // draw popup block
    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new keey-value pair")
            .borders(Borders::NONE)
            .style(Style::default().fg(Color::DarkGrey));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);
    }
}

/// helper function to create a centered rectangle using a certain percentage
/// of the available rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // cut rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(r); 
    
    // then cut the middle vertical piexe into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // return the middle chunk
}
