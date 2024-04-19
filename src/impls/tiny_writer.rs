use std::cmp::max;
use std::io::{Stdout, stdout};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use ratatui::backend::{CrosstermBackend};
use ratatui::Terminal;
use crate::impls::{tiny_writer, writer_mode};
use crate::structs::tiny_writer_struct::TinyWriter;

impl TinyWriter {
    pub fn new() -> Self{
        TinyWriter {
            lines : Vec::new(),
            current : String::new(),
            x : 0,
            y : 0,
            folded : false,
            mode : writer_mode::Mode::Edit,
            selected : Vec::new()
        }
    }

    pub fn rendering(&mut self, terminal: Terminal<CrosstermBackend<Stdout>>) {
        //
    }

    fn loop_flush(&mut self, start : u16, end : u16) {
        let mut count : u16 = 0;
        for index in start..=end {
            execute!(stdout(), MoveTo(0, count), Clear(ClearType::CurrentLine), Print(self.lines.get(index as usize).unwrap()));
            count += 1;
        }
    }

    pub fn rendering_up(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        // without save current line to the "lines"
        let height : usize = terminal.size().unwrap().height as usize;
        let size : usize = self.lines.len();
        let start : usize = if size > height { size - height } else { 0usize } ;
        let end : usize = if 0 < self.y { self.y - 1 } else { 0 };

        self.loop_flush(start as u16, end as u16);
        self.current = String::from(self.lines.get(self.y - 1).unwrap());
        self.y = end;
        execute!(stdout(), MoveTo(0, self.y as u16), Print(&self.current));
    }

    pub fn rendering_down(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        let height : usize = terminal.size().unwrap().height as usize;
        let size : usize = self.lines.len();
        let start : usize = if size + 2 > height { size - height + 1 } else { 0usize };
        let end : usize = self.y;

        self.loop_flush(start as u16, end as u16);
        execute!(stdout(), MoveTo(0, (self.y + 1) as u16), Clear(ClearType::CurrentLine));
        self.current = String::new();
        self.y += 1;
    }

    pub fn insert_new_line(&mut self, terminal : Terminal<CrosstermBackend<Stdout>>) {
        //
    }

    pub fn new_line(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        self.lines.push(String::from(&self.current)); // add current line to the history
        if self.lines.len() < terminal.size().unwrap().height as usize {
            self.current = String::new();
            self.y += 1;
            execute!(stdout(), MoveTo(0, self.y as u16));
            return;
        }
        self.rendering_down(terminal);
    }

    pub fn input(&mut self, c : &char, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        let width : usize = terminal.size().unwrap().width as usize;
        if self.current.len() + 1 > width {
            self.new_line(terminal);
            return;
        }

        self.current.push(*c);
        execute!(stdout(),
            Clear(ClearType::CurrentLine),
            MoveTo(0, self.y as u16),
            Print(&self.current));
    }

    pub fn delete(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        if !self.current.is_empty() {
            self.current.pop();
            let y : u16 = self.y as u16;
            execute!(stdout(), MoveTo(0, y), Clear(ClearType::CurrentLine), Print(&self.current));
            return;
        }

        if self.y < self.lines.len() { self.lines.remove(self.y); };
        execute!(stdout(), Clear(ClearType::CurrentLine));
        if self.y == 0 { return; };
        self.rendering_up(terminal);
    }
}