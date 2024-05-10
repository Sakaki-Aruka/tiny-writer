mod app;

use std::io::{stdout, Result, Stdout};
use std::panic::catch_unwind;
use crossterm::cursor::{MoveTo};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::{event, execute};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use crate::app::App;

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, EnableLineWrap,MoveTo(0, 0));
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    enable_raw_mode()?;

    let mut app: App = App::new();

    'main_loop: loop {
        while let Event::Key(KeyEvent {code, modifiers, kind, ..}) = event::read()? {
            if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('q') {
                break 'main_loop;
            }

            match code {
                KeyCode::Char(c) => {
                    app.input(&c, &mut terminal);
                }, KeyCode::Up => {
                    app.page_up(&mut terminal);
                }, KeyCode::Down => {
                    app.page_down(&mut terminal);
                }, KeyCode::Enter => {
                    app.enter(&mut terminal);
                }, KeyCode::Right => {
                    app.move_right(&mut terminal);
                }, KeyCode::Left => {
                    app.move_left(&mut terminal);
                },
                _ => ()
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen);
    disable_raw_mode()?;

    //debug
    dbg!(app.chars);

    Ok(())
}

fn display_coordinate(terminal : &mut Terminal<CrosstermBackend<Stdout>>, app : &App) {
    let (x, y) = terminal.get_cursor().unwrap();
    let index : String = String::from(app.index.to_string());
    let bottom_y = terminal.size().unwrap().height - 1;
    let mut temp : String = String::new();
    for c in &*app.chars {
        let d : char = if *c == '\n' { '%' } else { *c };
        temp.push(d);
    };
    execute!(
        stdout(),
        MoveTo(0, bottom_y),
        Clear(ClearType::CurrentLine),
        Print(format!("x={}, y={}, index={}, chars_len={}, chars={}", &x, &y, &index, app.chars.len(), &temp)));
    execute!(stdout(), MoveTo(x, y));
}
