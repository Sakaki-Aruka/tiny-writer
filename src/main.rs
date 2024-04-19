mod impls;
mod structs;

use std::cmp::max;
use std::fmt::format;
use crossterm::{event::{self, KeyCode, KeyEventKind}, terminal::
{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}, ExecutableCommand, execute, cursor, terminal};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use std::io::{stdout, Result, stdin};
use std::process::exit;
use std::thread::sleep;
use std::time;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::style::{Print, PrintStyledContent};
use crate::impls::actions::Actions;
use crate::impls::writer_mode::Mode;
use crate::structs::tiny_writer_struct::TinyWriter;


fn main() -> Result<()> {
    //
    execute!(stdout(), EnterAlternateScreen, MoveTo(0, 0))?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app : TinyWriter = TinyWriter::new();

    'main_loop : loop {
        while let Event::Key(KeyEvent {code, modifiers, kind, ..}) = event::read()? {

            //debug
            if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('q') {
                app.lines.push(String::from(&app.current));
                break 'main_loop;
            };

            match code {
                KeyCode::Char(c) => {
                    app.input(&c, &terminal);
                },
                KeyCode::Enter => {
                    app.new_line(&terminal);
                },
                KeyCode::Backspace => {
                    app.delete(&terminal);
                },
                _ => ()
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?; // in debug, disable.
    disable_raw_mode()?;

    //debug
    dbg!(&app.lines);
    dbg!(&app.lines.len());
    dbg!(&terminal.size().unwrap());

    Ok(())
}



