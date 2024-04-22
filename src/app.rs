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
}