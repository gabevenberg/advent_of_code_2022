use std::{
    cell::RefCell,
    collections::{hash_map, HashMap},
    ops::Deref,
    rc::{Rc, Weak},
};

pub type WeakDirRef = Weak<RefCell<Dir>>;

pub trait FileLike {
    fn get_parent(&self) -> Option<DirRef>;
    fn get_name(&self) -> String;
}

#[derive(Debug)]
pub struct DirRef(Rc<RefCell<Dir>>);

impl Deref for DirRef {
    type Target = Rc<RefCell<Dir>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DirRef {
    pub fn add_child_node(&mut self, mut node: Node) -> NodeRef {
        node.change_parent(self);
        let node_ref = NodeRef(Rc::new(RefCell::new(node)));
        Rc::clone(self)
            .borrow_mut()
            .children
            .push(NodeRef(Rc::clone(&node_ref)));
        node_ref
    }

    pub fn add_child_dir(&mut self, mut dir: Dir) -> DirRef {
        dir.parent = Some(Rc::downgrade(self));
        let dir_ref = DirRef(Rc::new(RefCell::new(dir)));
        Rc::clone(self).borrow_mut().children.push(
            Node::Dir(())
        );
        todo!()
    }
}

#[derive(Debug)]
pub struct Dir {
    name: String,
    parent: Option<WeakDirRef>,
    children: Vec<NodeRef>,
}

impl Dir {
    pub fn get_total_size(&self) -> usize {
        self.children
            .iter()
            .map(|n| -> usize {
                match Rc::clone(n).borrow().deref() {
                    Node::Dir(d) => d.get_total_size(),
                    Node::File(f) => f.size,
                }
            })
            .sum()
    }

    pub fn get_children(&self) -> impl Iterator<Item = NodeRef> + '_ {
        self.children.iter().map(|n| NodeRef(Rc::clone(n)))
    }
}

impl FileLike for Dir {
    fn get_parent(&self) -> Option<DirRef> {
        Some(DirRef(self.parent.as_ref()?.upgrade()?))
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
pub struct NodeRef(Rc<RefCell<Node>>);

impl Deref for NodeRef {
    type Target = Rc<RefCell<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum Node {
    Dir(Dir),
    File(File),
}

impl Node {
    fn change_parent(&mut self, new_parent: &DirRef) {
        match self {
            Node::Dir(d) => d.parent = Some(Rc::downgrade(new_parent)),
            Node::File(f) => f.parent = Rc::downgrade(new_parent),
        }
    }
}

impl FileLike for Node {
    fn get_parent(&self) -> Option<DirRef> {
        match self {
            Node::Dir(d) => d.get_parent(),
            Node::File(f) => f.get_parent(),
        }
    }
    fn get_name(&self) -> String {
        match self {
            Node::Dir(d) => d.get_name(),
            Node::File(f) => f.get_name(),
        }
    }
}

#[derive(Debug)]
pub struct File {
    name: String,
    parent: WeakDirRef,
    pub size: usize,
}

impl FileLike for File {
    fn get_parent(&self) -> Option<DirRef> {
        Some(DirRef(self.parent.upgrade()?))
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
