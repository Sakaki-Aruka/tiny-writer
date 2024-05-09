use std::io::{Stdout, stdout};
use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, ScrollDown, ScrollUp};
use crossterm::style::Print;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

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
        execute!(stdout(), Clear(ClearType::FromCursorDown));
        if self.index as usize + 1 >= self.chars.len() { return };
        let (x, y, width, height): (u16, u16, u16, u16) = Self::get_sizes(terminal);
        loop {
            if self.index == self.chars.len() as i64 - 1 { break };
            let (xx, yy, _, _): (u16, u16, u16, u16) = Self::get_sizes(terminal);
            let c: char = *self.chars.get(self.index as usize + 1).unwrap();
            if c == '\n' {
                if yy == height - 1 { break };
                self.index += 1;
                execute!(stdout(), MoveToNextLine(1));
                continue;
            }
            if xx == width - 1 && yy == height - 1 { break };
            execute!(stdout(), Print(&c));
            self.index += 1;
        }
        if move_back { execute!(stdout(), MoveTo(x, y)); }
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

        if bottom {
            execute!(stdout(), ScrollUp(1));
            if !has_more {
                execute!(stdout(), MoveToNextLine(1));
                return;
            }
            self.render_after_cursor(terminal, has_more);
        }
    }

    pub fn input(&mut self, c: &char, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        let (x, y, width, height) = Self::get_sizes(terminal);
        self.chars.insert(if self.index == -1 { 0 } else {self.index as usize}, *c);
        self.index += 1;

        let insert: bool = self.index != self.chars.len() as i64 - 1;
        let scroll: bool = x == width - 1 && y == height - 1;

        if !scroll && !insert{
            execute!(stdout(), Print(*c));
        } else if scroll {
            execute!(stdout(), ScrollUp(0), Print(" "), Print(*c));
            self.render_after_cursor(terminal, insert);
        } else {
            execute!(stdout(), Print(" "), Print(*c));
            self.render_after_cursor(terminal, true);
        }
    }

    pub fn move_right(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        //
    }

    pub fn move_left(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        //
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