use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileTreeError {
    #[error("Directory operation on file")]
    IsFile,
    #[error("File operation on directory")]
    IsDir,
}

type WeakNodeRef = Weak<RefCell<Node>>;

#[derive(Debug)]
pub struct NodeRef(Rc<RefCell<Node>>);

impl Deref for NodeRef {
    type Target = Rc<RefCell<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Rc<RefCell<Node>>> for NodeRef {
    fn from(value: Rc<RefCell<Node>>) -> Self {
        NodeRef(value)
    }
}

impl From<NodeRef> for Rc<RefCell<Node>> {
    fn from(value: NodeRef) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    parent: Option<WeakNodeRef>,
    contents: Contents,
}

impl Node {
    fn set_parent(&mut self, newparent: &NodeRef) {
        self.parent = Some(Rc::downgrade(newparent));
    }
    pub fn get_parent(&self) -> Option<NodeRef> {
        match &self.parent{
            Some(w) => Some(NodeRef(w.clone().upgrade()?)),
            None => None,
        }
    }
    pub fn get_size(&self) -> Result<usize, FileTreeError> {
        match self.contents {
            Contents::Size(s) => Ok(s),
            Contents::Children(_) => Err(FileTreeError::IsDir),
        }
    }
    pub fn set_size(&mut self, size: usize) -> Result<(), FileTreeError> {
        match self.contents {
            Contents::Size(ref mut s) => {
                *s = size;
                Ok(())
            }
            Contents::Children(_) => Err(FileTreeError::IsDir),
        }
    }
    pub fn get_children(&self) -> Result<impl Iterator<Item = NodeRef> + '_, FileTreeError> {
        match &self.contents {
            Contents::Size(_) => Err(FileTreeError::IsFile),
            Contents::Children(c) => Ok(c.iter().map(|n| NodeRef(Rc::clone(n)))),
        }
    }
}

#[derive(Debug)]
pub enum Contents {
    Size(usize),
    Children(Vec<NodeRef>),
}
