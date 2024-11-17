use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::utils;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn draw(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(98), Constraint::Percentage(3)])
        .split(frame.area());

    draw_instructions_layout(frame, app, layout[1]);
    draw_lrsgit_layout(frame, app, layout[0]);
}

fn draw_instructions_layout(frame: &mut Frame, app: &mut App, area: Rect) {
    let block_instructions = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);
    draw_block_instructions(frame, app, block_instructions[0]);
    draw_common_instructions(frame, app, block_instructions[1]);
}

fn draw_block_instructions(frame: &mut Frame, app: &mut App, area: Rect) {
    const FIRST_BLOCK_INSTRUCTIONS: [(&str, &str); 4] = [
        ("Quit", "q"),
        ("Cancel", "<esc>"),
        ("Check for update", "u"),
        ("Keybindings", "?"),
    ];
    let instructions = FIRST_BLOCK_INSTRUCTIONS
        .to_vec()
        .iter()
        .map(|&(event, event_key)| format!("{}: {}", event, event_key))
        .collect::<Vec<String>>()
        .join(" | ");

    let paragraph = Paragraph::new(instructions).style(Style::default().fg(Color::LightBlue));

    frame.render_widget(paragraph, area);
}

fn draw_common_instructions(frame: &mut Frame, app: &mut App, area: Rect) {
    let paragraph = Paragraph::new(format!("v{}", VERSION.unwrap()))
        .alignment(Alignment::Right)
        .style(Style::default().fg(Color::Blue));
    frame.render_widget(paragraph, area);
}

fn draw_lrsgit_layout(frame: &mut Frame, app: &mut App, area: Rect) {
    let blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(33), Constraint::Percentage(67)])
        .split(area);

    draw_lrsgit_left_layout(frame, app, blocks[0]);
    draw_lrsgit_right_layout(frame, app, blocks[1]);
}

fn draw_lrsgit_left_layout(frame: &mut Frame, app: &mut App, area: Rect) {
    let blocks_left_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ])
        .split(area);
    draw_status_block(frame, app, blocks_left_column[0]);
    draw_files_block(frame, app, blocks_left_column[1]);
    draw_localbranches_block(frame, app, blocks_left_column[2]);
    draw_commits_block(frame, app, blocks_left_column[3]);
    draw_stash_block(frame, app, blocks_left_column[4]);
}

fn draw_lrsgit_right_layout(frame: &mut Frame, app: &mut App, area: Rect) {
    let blocks_right_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);
    draw_main_block(frame, app, blocks_right_column[0]);
    draw_commandlog_block(frame, app, blocks_right_column[1]);
}

fn draw_main_block(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" Status");

    let block = Block::bordered()
        .title(title)
        .border_style(if app.current_block == utils::Blocks::Main {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

fn draw_commandlog_block(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" Command Log");

    let block = Block::bordered()
        .title(title)
        .border_style(if app.current_block == utils::Blocks::Logs {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

fn draw_status_block(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" [1]-Status");

    let block = Block::bordered()
        .title(title)
        .border_style(if app.current_block == utils::Blocks::First {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

fn draw_files_block(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" [2]-Files");

    let block = Block::bordered()
        .title(title)
        .border_style(if app.current_block == utils::Blocks::Second {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

fn draw_localbranches_block(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" [3]-Local Branches");

    let block = Block::bordered()
        .title(title)
        .border_style(if app.current_block == utils::Blocks::Third {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

fn draw_commits_block(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" [4]-Commits");

    let block = Block::bordered()
        .title(title)
        .border_style(if app.current_block == utils::Blocks::Fourth {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

fn draw_stash_block(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = Title::from(" [5]-Stash");

    let block = Block::bordered()
        .title(title)
        .border_style(if app.current_block == utils::Blocks::Fifth {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}
