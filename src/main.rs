mod app;

use std::io::{stdout, Result, Stdout};
use crossterm::cursor::{MoveLeft, MoveRight, MoveTo, MoveToNextLine, MoveToPreviousLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::{event, execute};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown, ScrollUp};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use crate::app::App;

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, EnableLineWrap,MoveTo(0, 0));
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    enable_raw_mode()?;

    let mut app: App = App::new();

    let mut terminal_width_buffer: u16 = terminal.size()?.width;
    let mut terminal_height_buffer: u16 = terminal.size()?.height;


    // //debug
    // for i in 1..(terminal_width_buffer * (terminal_height_buffer-1)) {
    //     execute!(stdout(), Print("A"));
    // }

    'main_loop: loop {
        while let Event::Key(KeyEvent {code, modifiers, kind, ..}) = event::read()? {
            if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('q') {
                break 'main_loop;
            }

            let (x, y) : (u16, u16) = terminal.get_cursor().unwrap();
            let width : u16 = terminal.size()?.width;
            let height : u16 = terminal.size()?.height;

            match code {
                KeyCode::Char(c) => {
                    app.input(&c, &mut terminal);
                }, KeyCode::Up => {
                    app.page_up(&mut terminal);
                }, KeyCode::Down => {
                    app.page_down(&mut terminal);
                }, _ => ()
            }
        }
    }

    //
    // 'main_loop: loop {
    //     while let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event::read()? {
    //         if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('q') {
    //             break 'main_loop;
    //         }
    //
    //         let (x, y) = terminal.get_cursor()?;
    //         let x_last: bool = x == terminal_width_buffer - 1;
    //         let y_last: bool = y == terminal_height_buffer - 1;
    //         let width : u16 = terminal.size()?.width;
    //         let height : u16 = terminal.size()?.height;
    //
    //         match code {
    //             KeyCode::Char(c) => {
    //                 app.chars.insert(*&app.index as usize, *&c);
    //                 app.index += 1;
    //
    //                 if x_last && y_last { execute!(stdout(), ScrollUp(1)); };
    //                 if x_last {
    //                     execute!(stdout(), MoveToNextLine(1));
    //                     app.chars.insert(app.index as usize, '\n');
    //                     app.index += 1;
    //                 };
    //                 execute!(stdout(), Print(c));
    //             },
    //             KeyCode::Enter => {
    //                 if y_last { execute!(stdout(), ScrollUp(1)); };
    //                 execute!(stdout(), MoveToNextLine(1));
    //                 app.chars.insert(app.index as usize, '\n');
    //                 app.index += 1;
    //             },
    //             KeyCode::Left => {
    //                 if x == 0 && y == 0 && app.index == 0 { continue; };
    //                 if x == 0 && y == 0 { execute!(stdout(), ScrollDown(1)); }
    //                 if x == 0 {
    //                     app.index -= 1;
    //                     execute!(stdout(), MoveToPreviousLine(1));
    //                     let previous_line : Option<String> = app.get_before_line(&width);
    //                     if previous_line.is_some() {
    //                         execute!(stdout(), Print(previous_line.unwrap()));
    //                     }
    //                 }
    //                 else {
    //                     execute!(stdout(), MoveLeft(1));
    //                     app.index -= 1;
    //                 };
    //             },
    //             KeyCode::Right => {
    //                 let current: Option<&char> = app.chars.get(app.index as usize);
    //                 if current.is_none() { continue; };
    //                 let current: char = *current.unwrap();
    //                 let current_is_lf: bool = current == '\n';
    //                 if x_last || current_is_lf {
    //                     app.index += 1;
    //                     if y_last { execute!(stdout(), ScrollUp(1)); }
    //                     execute!(stdout(), MoveToNextLine(1));
    //                     let displays : String = app.get_element_after_cursor(&width, &x).unwrap_or(String::from(""));
    //                     if displays.is_empty() { continue; };
    //                     execute!(stdout(), Clear(ClearType::CurrentLine), Print(&displays));
    //                     let y : u16 = terminal.get_cursor().unwrap().1;
    //                     execute!(stdout(), MoveTo(0, y));
    //                 } else {
    //                     execute!(stdout(), MoveRight(1));
    //                     app.index += 1;
    //                 };
    //             }
    //             KeyCode::Backspace => {
    //                 //
    //             },
    //             _ => (),
    //         }
    //
    //         //debug
    //         //display_coordinate(&mut terminal, &app);
    //     }
    // }
    //execute!(stdout(), LeaveAlternateScreen);
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
