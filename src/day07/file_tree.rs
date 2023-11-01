#![allow(unused)]
use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

// #[derive(Debug)]
// pub struct NodeRef(Rc<RefCell<Node>>);

#[derive(Debug)]
pub struct FileRef(Rc<RefCell<File>>);

#[derive(Debug)]
pub struct DirRef(Rc<RefCell<Dir>>);
#[derive(Debug)]
pub struct WeakDirRef(Weak<RefCell<Dir>>);

#[derive(Debug)]
pub enum Node {
    Dir(DirRef),
    File(FileRef),
}

#[derive(Debug)]
pub struct File {
    parent: WeakDirRef,
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Default)]
pub struct Dir {
    parent: Option<WeakDirRef>,
    pub name: String,
    children: Vec<Node>,
}

impl DirRef {
    //impling it on the ref makes the recursion a bit easier.
    fn total_size(&self) -> usize {
        self.0
            .borrow()
            .deref()
            .children
            .iter()
            .map(|f| match f {
                Node::Dir(dir) => dir.total_size(),
                Node::File(file) => file.0.borrow().size,
            })
            .sum()
    }
    fn new(name: String) -> Self {
        DirRef(Rc::new(RefCell::new(Dir {
            parent: None,
            name,
            children: Vec::new(),
        })))
    }

    //needs to be impled on the ref because of the need to get a weak backreference to self.
    fn add(&self, node: Node) {
        match node {
            Node::Dir(ref dir) => {
                dir.0.borrow_mut().parent = Some(WeakDirRef(Rc::downgrade(&self.0)))
            }
            Node::File(ref file) => file.0.borrow_mut().parent = WeakDirRef(Rc::downgrade(&self.0)),
        }
        self.0.borrow_mut().children.push(node);
    }
}

impl File {
    fn get_parent(&self)-> &WeakDirRef {
        &self.parent
    }
}

impl Dir {
    fn get_children(&self) -> &[Node] {
        &self.children
    }

    fn get_parent(&self) -> &Option<WeakDirRef> {
        &self.parent
    }
}
