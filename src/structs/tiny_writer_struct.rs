use crate::impls::writer_mode;

pub struct TinyWriter {
    pub lines: Vec<String>,
    pub current : String,
    pub x : usize,
    pub y : usize,
    pub folded : bool,
    pub mode : writer_mode::Mode,
    pub selected : Vec<String>,
}