mod impls;
mod structs;

use std::cmp::max;
use crossterm::{event::{self, KeyCode, KeyEventKind}, terminal::
{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}, ExecutableCommand, execute, cursor, terminal};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use std::io::{stdout, Result, stdin};
use std::process::exit;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::style::{Print, PrintStyledContent};
use crate::impls::actions::Actions;
use crate::impls::writer_mode::Mode;
use crate::structs::tiny_writer_struct::TinyWriter;


fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    //enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app : TinyWriter = TinyWriter::new();
    'main_loop : loop {
        while let Event::Key(KeyEvent {code, modifiers, kind, ..}) = event::read()? {
            //
            break 'main_loop;
        }
    }

    let input : fn(TinyWriter) = |mut app : TinyWriter| {
        //
    };

    let rendering_up : fn(TinyWriter) = |mut app : TinyWriter| {
        let width : usize = terminal.size().unwrap().width as usize;
        let height : usize = terminal.size().unwrap().height as usize;
        let start : usize = max(0usize, app.y - 1 - height - 1);
        let end : usize = app.y;
        let mut count : usize = 0;
        for index in start..end {
            execute!(stdout(), MoveTo(0, count as u16),Print(&app.lines.get(index)))?;
            count += 1;
        }
        execute!(stdout(), MoveTo(app.lines.get(height - 1).unwrap().len() as u16, height as u16- 1u16))?;
    };

    let rendering_down : fn(TinyWriter) = |mut app : TinyWriter| {
        //
    };


    // let mut x : u16 = 0;
    // let mut y : u16 = 1u16;
    // let mut line : String = String::new();
    // let mut lines : Vec<String> = Vec::new();
    // 'main_loop : loop {
    //
    //     while let Event::Key(KeyEvent {code, modifiers,kind, ..}) = event::read()? {
    //         if kind != KeyEventKind::Press { continue };
    //
    //         match modifiers {
    //             KeyModifiers::CONTROL => {
    //                 match code {
    //                     KeyCode::Char(c) => if c == 'q' { break 'main_loop } ,
    //                     _ => ()
    //                 }
    //             },
    //             _ => ()
    //         }
    //
    //         match code {
    //             KeyCode::Char(c) => {
    //                 line.push(c);
    //                 execute!(stdout(), Clear(ClearType::CurrentLine), MoveTo(0, y), Print(&line))?;
    //             },
    //             KeyCode::Enter => {
    //                 y += 1;
    //                 execute!(stdout(), MoveTo(0, y))?;
    //                 &lines.push(String::from(&line));
    //                 line.clear();
    //             },
    //             KeyCode::Backspace => {
    //                 // if (&line).len() == 0usize { continue };
    //                 line.pop();
    //                 if (&line).len() == 0usize {
    //                     execute!(stdout(), Clear(ClearType::CurrentLine))?;
    //                     y -= 1;
    //                     lines.remove(y as usize - 1usize);
    //                 }
    //                 execute!(stdout(), Clear(ClearType::CurrentLine), MoveTo(0, y), Print(&line))?;
    //             },
    //             _ => continue,
    //         }
    //     }
    //}

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}



