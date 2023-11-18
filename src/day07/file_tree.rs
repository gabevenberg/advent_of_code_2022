use std::{
    cell::RefCell,
    fmt::Display,
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
    #[error("File not found")]
    FileNotFound,
    #[error("File already exists")]
    FileAlreadyExists,
}

type WeakNodeRef = Weak<RefCell<Node>>;

#[derive(Debug)]
pub struct NodeRef(pub Rc<RefCell<Node>>);

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

impl Display for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

impl NodeRef {
    pub fn add_node(&mut self, mut node: Node) -> Result<NodeRef, FileTreeError> {
        node.set_parent(self);
        self.borrow_mut().add_children(node)
    }
    pub fn new_dir(name: String) -> NodeRef {
        NodeRef(Rc::new(RefCell::new(Node::new_dir(name))))
    }
    pub fn add_file(&mut self, name: String, size: usize) -> Result<NodeRef, FileTreeError> {
        self.add_node(Node::new_file(name, size))
    }
    pub fn add_dir(&mut self, name: String) -> Result<NodeRef, FileTreeError> {
        self.add_node(Node::new_dir(name))
    }
    pub fn get_all_dirs(&self) -> Vec<NodeRef> {
        let mut ret = Vec::new();
        match &self.borrow().contents {
            Contents::Size(_) => {}
            Contents::Children(c) => {
                ret.push(NodeRef(Rc::clone(self)));
                for node in c {
                    ret.append(&mut node.get_all_dirs())
                }
            }
        };
        ret
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    parent: Option<WeakNodeRef>,
    contents: Contents,
}

impl Node {
    pub fn new_dir(name: String) -> Node {
        Node {
            name,
            parent: None,
            contents: Contents::Children(Vec::new()),
        }
    }
    pub fn new_file(name: String, size: usize) -> Node {
        Node {
            name,
            parent: None,
            contents: Contents::Size(size),
        }
    }
    pub fn get_total_size(&self) -> usize {
        match &self.contents {
            Contents::Size(s) => *s,
            Contents::Children(c) => c.iter().map(|f| f.borrow().get_total_size()).sum(),
        }
    }
    fn set_parent(&mut self, newparent: &NodeRef) {
        self.parent = Some(Rc::downgrade(newparent));
    }
    // does not set the parent, needs to be done on the NodeRef. (this is why this func isnt pub).
    // takes onwership of the node to make sure its not owned by another tree.
    fn add_children(&mut self, node: Node) -> Result<NodeRef, FileTreeError> {
        match self.contents {
            Contents::Size(_) => Err(FileTreeError::IsFile),
            Contents::Children(ref mut c) => {
                for file in c.iter() {
                    if file.borrow().name == node.name {
                        return Err(FileTreeError::FileAlreadyExists);
                    }
                }
                let rc = Rc::new(RefCell::new(node));
                c.push(NodeRef(Rc::clone(&rc)));
                Ok(NodeRef(Rc::clone(&rc)))
            }
        }
    }
    pub fn remove_child_by_name(&mut self, name: &str) -> Result<(), FileTreeError> {
        match self.contents {
            Contents::Size(_) => Err(FileTreeError::IsFile),
            Contents::Children(ref mut c) => {
                for (i, file) in c.iter().enumerate() {
                    if file.borrow().name == name {
                        c.remove(i);
                        return Ok(());
                    }
                }
                Err(FileTreeError::FileNotFound)
            }
        }
    }
    pub fn get_parent(&self) -> Option<NodeRef> {
        match &self.parent {
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
    pub fn get_child_by_name(&self, name: &str) -> Result<NodeRef, FileTreeError> {
        match &self.contents {
            Contents::Size(_) => Err(FileTreeError::IsFile),
            Contents::Children(c) => {
                for file in c.iter() {
                    if file.borrow().name == name {
                        return Ok(NodeRef(Rc::clone(file)));
                    }
                }
                Err(FileTreeError::FileNotFound)
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} {}", self.name, self.contents)
    }
}

#[derive(Debug)]
pub enum Contents {
    Size(usize),
    Children(Vec<NodeRef>),
}

impl Display for Contents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Contents::Size(s) => write!(f, "(file, size = {})", s),
            Contents::Children(c) => {
                writeln!(f, "(dir)").expect("I have no clue how this could fail");
                for node in c {
                    //padding
                    for line in format!("{}", node).lines() {
                        writeln!(f, " {line}")?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_construction() {
        let mut root = NodeRef::new_dir("/".to_string());
        let mut cursor = root.add_node(Node::new_dir("a".to_string())).unwrap();
        cursor = cursor.add_dir("e".to_string()).unwrap();
        cursor.add_file("i".to_string(), 584).unwrap();
        cursor = Rc::clone(cursor.deref()).borrow().get_parent().unwrap();
        cursor.add_file("f".to_string(), 29116).unwrap();
        cursor.add_file("g".to_string(), 2557).unwrap();
        cursor.add_file("h.lst".to_string(), 62596).unwrap();
        cursor = Rc::clone(cursor.deref()).borrow().get_parent().unwrap();
        cursor.add_file("b.txt".to_string(), 14848514).unwrap();
        cursor.add_file("c.dat".to_string(), 8504156).unwrap();
        cursor = cursor.add_dir("d".to_string()).unwrap();
        cursor.add_file("j".to_string(), 4060174).unwrap();
        cursor.add_file("d.log".to_string(), 8033020).unwrap();
        cursor.add_file("d.ext".to_string(), 5626152).unwrap();
        cursor.add_file("k".to_string(), 7214296).unwrap();
        assert_eq!(Rc::clone(root.deref()).borrow().get_total_size(), 48381165);
        println!("{}", root);
    }

    #[test]
    fn test_size_calcs() {
        let mut root = NodeRef::new_dir("/".to_string());
        let mut dirs: Vec<NodeRef> = Vec::new();
        let mut cursor = root.add_dir("a".to_string()).unwrap();
        dirs.push(NodeRef(Rc::clone(&cursor)));
        cursor = cursor.add_dir("e".to_string()).unwrap();
        dirs.push(NodeRef(Rc::clone(&cursor)));
        cursor.add_file("i".to_string(), 584).unwrap();
        cursor = Rc::clone(cursor.deref()).borrow().get_parent().unwrap();
        cursor.add_file("f".to_string(), 29116).unwrap();
        cursor.add_file("g".to_string(), 2557).unwrap();
        cursor.add_file("h.lst".to_string(), 62596).unwrap();
        cursor = Rc::clone(cursor.deref()).borrow().get_parent().unwrap();
        cursor.add_file("b.txt".to_string(), 14848514).unwrap();
        cursor.add_file("c.dat".to_string(), 8504156).unwrap();
        cursor = cursor.add_node(Node::new_dir("d".to_string())).unwrap();
        dirs.push(NodeRef(Rc::clone(&cursor)));
        cursor.add_file("j".to_string(), 4060174).unwrap();
        cursor.add_file("d.log".to_string(), 8033020).unwrap();
        cursor.add_file("d.ext".to_string(), 5626152).unwrap();
        cursor.add_file("k".to_string(), 7214296).unwrap();
        assert_eq!(Rc::clone(root.deref()).borrow().get_total_size(), 48381165);
        assert_eq!(Rc::clone(dirs[0].deref()).borrow().get_total_size(), 94853);
        assert_eq!(Rc::clone(dirs[1].deref()).borrow().get_total_size(), 584);
        assert_eq!(
            Rc::clone(dirs[2].deref()).borrow().get_total_size(),
            24933642
        );
    }
    #[test]
    fn test_get_all_dirs() {
        let mut root = NodeRef::new_dir("/".to_string());
        let mut cursor = root.add_dir("a".to_string()).unwrap();
        cursor = cursor.add_dir("e".to_string()).unwrap();
        cursor.add_file("i".to_string(), 584).unwrap();
        cursor = Rc::clone(cursor.deref()).borrow().get_parent().unwrap();
        cursor.add_file("f".to_string(), 29116).unwrap();
        cursor.add_file("g".to_string(), 2557).unwrap();
        cursor.add_file("h.lst".to_string(), 62596).unwrap();
        cursor = Rc::clone(cursor.deref()).borrow().get_parent().unwrap();
        cursor.add_file("b.txt".to_string(), 14848514).unwrap();
        cursor.add_file("c.dat".to_string(), 8504156).unwrap();
        cursor = cursor.add_node(Node::new_dir("d".to_string())).unwrap();
        cursor.add_file("j".to_string(), 4060174).unwrap();
        cursor.add_file("d.log".to_string(), 8033020).unwrap();
        cursor.add_file("d.ext".to_string(), 5626152).unwrap();
        cursor.add_file("k".to_string(), 7214296).unwrap();
        let dirs = root.get_all_dirs();
        let sizes: Vec<usize> = dirs.iter().map(|d| d.borrow().get_total_size()).collect();
        println!("{:?}", sizes);
        assert_eq!(Rc::clone(root.deref()).borrow().get_total_size(), 48381165);
        assert_eq!(
            Rc::clone(dirs[0].deref()).borrow().get_total_size(),
            48381165
        );
        assert_eq!(Rc::clone(dirs[1].deref()).borrow().get_total_size(), 94853);
        assert_eq!(Rc::clone(dirs[2].deref()).borrow().get_total_size(), 584);
        assert_eq!(
            Rc::clone(dirs[3].deref()).borrow().get_total_size(),
            24933642
        );
    }
}
