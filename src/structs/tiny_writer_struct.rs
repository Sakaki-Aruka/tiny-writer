use std::collections::HashMap;
use crate::impls::writer_mode;

#[derive(Clone)]
pub struct TinyWriter {
    pub lines: Vec<String>,
    pub current : String,
    pub x : usize,
    pub y : usize,
    pub mode : writer_mode::Mode,
    pub selected : Vec<String>,
    pub folded_list : Vec<usize>,
}