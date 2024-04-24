use std::cmp::{max, min};
use std::io::stdout;
use crossterm::cursor::{MoveRight, MoveTo, MoveToNextLine};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

pub struct App {
    pub current_page_start_index : u64,
    pub current_line_start_index : u64,
    pub chars : Vec<char>,
    pub index : u64,
}

impl App {
    pub fn new() -> Self {
        App {
            current_page_start_index : 0,
            current_line_start_index : 0,
            chars : Vec::new(),
            index : 0,
        }
    }

    pub fn get_before_line(&self, width : &u16) -> Option<String> {
        if self.index == 0 { return None; };
        let mut result : String = String::new();
        let mut temporary_chars : Vec<char> = Vec::new();
        let mut count: u16 = 0;
        for index in (0..self.index).rev() {
            if count >= *width { break; };
            let c : char = *self.chars.get(index as usize).unwrap();
            if c == '\n' { break; };
            temporary_chars.insert(0, c);
            count += 1;
        }
        if temporary_chars.is_empty() { return None; };
        for c in temporary_chars { result.push(c); };

        Some(result)
    }

    pub fn get_chars_after_cursor(&self, width : &u16, x : &u16) -> Option<String> {
        if *width <= *x + 1 { return None; };
        let limit : u16 = *width - *x - 1;
        let mut current : u16 = self.index as u16;
        let mut result : String = String::new();
        for diff in 1..= min((*width - limit) as usize, self.chars.len() - self.index as usize) {
            let index : usize = current as usize;
            if index >= self.chars.len() { break; };

            //debug
            let c : char = *self.chars.get(index).expect(format!("index={}, limit={}, loop_end={}, diff={}", &index, &limit, &(*width-limit), &diff).as_str());

            //let c : char = *self.chars.get(index).unwrap();
            if c == '\n' { break;} ;
            result.push(c);
            current += 1;
        }

        if result.is_empty() { None } else { Some(result) }
    }

    pub fn rendering(&self, width : &u16, height : &u16, start : &u64) {
        if *start >= self.chars.len() as u64 { return; };
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All));
        let mut current_x : u16 = 0;
        let mut current_y : u16 = 0;
        for index in *start as usize..self.chars.len() {
            let x_last : bool = current_x == *width - 1;
            let y_last : bool = current_y == *height - 1;
            if x_last && y_last { break; };
            let c : char = *self.chars.get(index).unwrap();
            if c == '\n' {
                if y_last { break; };
                execute!(stdout(), MoveToNextLine(1));
                continue;
            }
            if x_last { execute!(stdout(), MoveToNextLine(1)); }
            execute!(stdout(), Print(c), MoveRight(1));
        }
    }
}