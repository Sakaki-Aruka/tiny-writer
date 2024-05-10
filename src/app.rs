use std::io::{Stdout, stdout, Write};
use crossterm::cursor::{Hide, MoveLeft, MoveRight, MoveTo, MoveToNextLine, MoveToPreviousLine, Show};
use crossterm::{execute, queue};
use crossterm::terminal::{Clear, ClearType, ScrollDown, ScrollUp};
use crossterm::style::Print;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use unicode_width::*;

pub struct App {
    pub chars : Vec<char>,
    pub index : i64,
    pub left_top : i64,
}

impl App {
    pub fn new() -> Self {
        App {
            chars : Vec::new(),
            index : -1,
            left_top : -1
        }
    }


    fn get_sizes(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> (u16, u16, u16, u16) {
        let x: u16 = terminal.get_cursor().unwrap().0;
        let y: u16 = terminal.get_cursor().unwrap().1;
        let width: u16 = terminal.size().unwrap().width;
        let height: u16 = terminal.size().unwrap().height;
        (x, y, width, height)
    }

    pub fn render_after_cursor(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, move_back: bool) {
        let (x, y, width, height): (u16, u16, u16, u16) = Self::get_sizes(terminal);
        execute!(stdout(), Clear(ClearType::FromCursorDown), MoveTo(x, y));
        if self.index as usize >= self.chars.len() { return };
        queue!(stdout(), Hide); // cursor hide
        let mut add: usize = 0;
        loop {
            let (xx, yy, _, _) = Self::get_sizes(terminal);
            let index: usize = self.index as usize + add;
            let x_last: bool = xx == width - 1;
            let y_last: bool = yy == height - 1;
            if index >= self.chars.len() - 1 { break };
            let c: char = *self.chars.get(index).unwrap();
            if y_last && (x_last || xx + c.width_cjk().unwrap() as u16 >= width) { break };
            add += 1;
            if c == '\n' {
                queue!(stdout(), MoveToNextLine(1), Clear(ClearType::CurrentLine));
                continue
            } else { queue!(stdout(), Print(c)); }
        }
        if move_back { queue!(stdout(), MoveTo(x, y)); }
        queue!(stdout(), Show);
        stdout().flush().unwrap();
    }

    pub fn enter(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        let (x, y, width, height): (u16, u16, u16, u16) = Self::get_sizes(terminal);
        self.chars.insert(if self.index == -1 { 0 } else { self.index as usize }, '\n');
        self.index += 1;
        let has_more: bool = self.index != self.chars.len() as i64 - 1;
        let bottom: bool = y >= height - 1;
        if !bottom && !has_more {
            execute!(stdout(), MoveToNextLine(1));
            return;
        }

        if bottom { execute!(stdout(), ScrollUp(1)); }
        execute!(stdout(), Clear(ClearType::FromCursorDown), MoveToNextLine(1));
        if !has_more { return; }
        self.render_after_cursor(terminal, true);
    }

    pub fn input(&mut self, c: &char, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        let (x, y, width, height) = Self::get_sizes(terminal);
        self.chars.insert(if self.index == -1 { 0 } else { self.index as usize }, *c);
        self.index += 1;

        let insert: bool = self.index != self.chars.len() as i64 - 1;
        let scroll: bool = x == width - 1 && y == height - 1;

        if !scroll && !insert{
            execute!(stdout(), Print(*c));
        } else if scroll {
            execute!(stdout(), ScrollUp(0), Print(*c));
            self.render_after_cursor(terminal, insert);
        } else {
            execute!(stdout(), Print(*c));
            self.render_after_cursor(terminal, true);
        }
    }

    pub fn move_right(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        let (x, y, width, height): (u16, u16, u16, u16) = Self::get_sizes(terminal);
        if self.index >= self.chars.len() as i64 - 1 { return };
        let c: char = *self.chars.get(self.index as usize).unwrap();
        let y_last: bool = y == height - 1;
        let x_last: bool = x == width - 1;

        self.index += 1;
        if !y_last && !x_last {
            if c != '\n' {
                execute!(stdout(), MoveRight(1));//Print(c));
            } else {
                // '\n'
                execute!(stdout(), MoveToNextLine(1));
            }
        } else if x_last {
            if y_last {
                execute!(stdout(), ScrollUp(1));
                self.render_after_cursor(terminal, true);
            } else {
                execute!(stdout(), MoveToNextLine(1));
            }
        }
    }

    fn get_above_line_chars(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Option<Vec<char>> {
        // from the cursor to the latest \n.
        // e.g.) "hello\n world" -> " world"
        if self.index <= 0 { return None };
        let mut result: Vec<char> = Vec::new();
        let mut count: usize = 0;
        let (x, y, width, height): (u16, u16, u16, u16) = Self::get_sizes(terminal);
        for i in (0..self.index - 1).rev() {
            let c: char = *self.chars.get(i as usize).unwrap();
            if c == '\n' || count + c.width_cjk().unwrap() >= width as usize { break };
            result.push(c);
            count += c.width_cjk().unwrap();
        }
        if result.is_empty() { return None };
        result.reverse();
        Some(result)
    }

    pub fn move_left(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        let (x, y, width, height): (u16, u16, u16, u16) = Self::get_sizes(terminal);
        if self.index <= 0 {
            if self.index == 0 { execute!(stdout(), MoveLeft(1)); }
            return
        };
        let highest: bool = y == height - 1;
        let most_left: bool = x == 0;

        self.index -= 1;
        if highest && most_left {
            // scroll up
            let chars: Option<Vec<char>> = self.get_above_line_chars(terminal);
            if chars.is_none() { return };
            execute!(stdout(), ScrollDown(1), MoveToPreviousLine(1));
            for c in chars.unwrap() {
                execute!(stdout(), Print(c));
            }
        } else if most_left {
            let above_line: Option<Vec<char>> = self.get_above_line_chars(terminal);
            let x: u16 = if above_line.is_none() { 0 } else { above_line.unwrap().len() as u16 };
            let y: u16 = y - 1;
            execute!(stdout(), MoveTo(x, y));
        } else {
            // !most_left
            execute!(stdout(), MoveLeft(1));
        }
    }

    pub fn page_up(&mut self, terminal : &mut Terminal<CrosstermBackend<Stdout>>) {
    }

    pub fn page_down(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        //
    }

    pub fn insert_enter(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        //
    }

    pub fn delete(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        //
    }
}