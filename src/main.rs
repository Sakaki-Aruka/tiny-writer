mod app;

use std::io::{stdout, Result};
use crossterm::cursor::{MoveLeft, MoveRight, MoveTo, MoveToNextLine, MoveToPreviousLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::{event, execute};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown, ScrollUp};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use crate::app::App;

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, EnableLineWrap,MoveTo(0, 0));
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    enable_raw_mode()?;

    let mut app: App = App::new();

    let mut terminal_width_buffer: u16 = terminal.size()?.width;
    let mut terminal_height_buffer: u16 = terminal.size()?.height;
    let mut index: u64 = 0;
    let mut current_line_start_index: u64 = 0;
    let mut chars: Vec<char> = Vec::new();

    'main_loop: loop {
        while let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event::read()? {
            if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('q') {
                break 'main_loop;
            }

            let (x, y) = terminal.get_cursor()?;
            let x_last = x == terminal_width_buffer - 1;
            let y_last = y == terminal_height_buffer - 1;

            match code {
                KeyCode::Char(c) => {
                    if x_last && y_last { execute!(stdout(), ScrollUp(1)); };
                    if x_last { execute!(stdout(), MoveToNextLine(1)); };
                    execute!(stdout(), Print(c));
                },
                KeyCode::Enter => {
                    if y_last { execute!(stdout(), ScrollUp(1)); };
                    execute!(stdout(), MoveToNextLine(1));
                },
                KeyCode::Left => {
                    if x == 0 { execute!(stdout(), MoveToPreviousLine(1)); } else { execute!(stdout(), MoveLeft(1)); };
                },
                KeyCode::Right => {
                    if x_last {
                        if y_last { execute!(stdout(), ScrollUp(1)); }
                        execute!(stdout(), MoveToNextLine(1));
                    } else { execute!(stdout(), MoveRight(1)); };
                },
                KeyCode::Backspace => {
                    //
                },
                _ => (),
            }
        }
    }
    execute!(stdout(), LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}
