#![allow(unused)]
use std::{rc::{Weak, Rc}, cell::RefCell};

#[derive(Debug)]
pub struct Dir {
    parent: Option<Weak<RefCell<Dir>>>,
    name: String,
    children: Vec<Rc<RefCell<FileOrDir>>>
}

#[derive(Debug)]
pub struct File{
    dir: Weak<RefCell<Dir>>,
    size: usize,
    name: String
}

#[derive(Debug)]
pub enum FileOrDir{
    File(File),
    Dir(Dir),
}
