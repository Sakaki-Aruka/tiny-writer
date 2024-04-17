use crate::impls::{tiny_writer, writer_mode};
use crate::structs::tiny_writer_struct::TinyWriter;

impl TinyWriter {
    pub fn new() -> Self{
        TinyWriter {
            lines : Vec::new(),
            current : String::new(),
            x : 0,
            y : 1,
            folded : false,
            mode : writer_mode::Mode::Edit,
            selected : Vec::new()
        }
    }
}