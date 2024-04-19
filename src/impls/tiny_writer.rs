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

    pub fn rendering_up(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        // without save current line to the "lines"
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
        let height : usize = terminal.size().unwrap().height as usize;
        let start : usize = max(0usize, self.y - 1 - height);
        let end : usize = self.y - 3;
        let mut count : usize = 0;

        for index in start..end {
            let temp = String::from(self.lines.get(index).unwrap());
            execute!(stdout(), MoveTo(0, count as u16), Print(temp));
            count += 1;
        }
        execute!(stdout(), MoveTo(self.lines.get(height - 1).unwrap().len() as u16, height as u16 - 1u16));
        self.current = String::from(self.lines.get(end - 1).unwrap());
        self.y = end;
    }

    pub fn rendering_down(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        let height : usize = terminal.size().unwrap().height as usize;

        let start : usize = if self.y > height { self.y - height + 2 } else { 0usize };
        let end : usize = self.y;

        for index in start..=end {
            let y : usize = self.y % height;
            execute!(stdout(), MoveTo(0, y as u16), Clear(ClearType::CurrentLine),Print(self.lines.get(index).unwrap()));
        }
        execute!(stdout(), MoveTo(0, (self.y + 1) as u16));
        self.current = String::new();
        self.y += 1;
    }

    pub fn new_line(&mut self, terminal : &Terminal<CrosstermBackend<Stdout>>) {
        self.lines.push(String::from(&self.current)); // add current line to the history
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
        let width : usize = terminal.size().unwrap().width as usize;
        if self.current.is_empty() && self.current.len() < width {
            self.current.pop();
            return;
        }

        self.rendering_up(terminal);
        let x : u16 = self.current.len() as u16;
        let y : u16 = self.y as u16;
        execute!(stdout(), MoveTo(x, y));
    }
}