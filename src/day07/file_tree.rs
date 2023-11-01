#![allow(unused)]
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    iter,
    ops::Deref,
    rc::{Rc, Weak},
    slice::Iter,
};

//these cant be just a type, as methods need to be impl'd on them.
pub struct FileRef(Rc<RefCell<File>>);
pub struct DirRef(Rc<RefCell<Dir>>);
pub struct WeakDirRef(Weak<RefCell<Dir>>);

pub enum Node {
    Dir(DirRef),
    File(FileRef),
}

pub struct File {
    parent: WeakDirRef,
    pub name: String,
    pub size: usize,
}

#[derive(Default)]
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
    fn add(&self, node: Node) -> Node {
        let ret;
        match node {
            Node::Dir(ref dir) => {
                dir.0.borrow_mut().parent = Some(WeakDirRef(Rc::downgrade(&self.0)));
                ret = Node::Dir(DirRef(Rc::clone(&dir.0)));
            }
            Node::File(ref file) => {
                file.0.borrow_mut().parent = WeakDirRef(Rc::downgrade(&self.0));
                ret = Node::File(FileRef(Rc::clone(&file.0)));
            }
        }
        self.0.borrow_mut().children.push(node);
        ret
    }

    //because a file always has a parent, it needs to be created off of a node.
    fn new_file(&self, name: String, size: usize) -> FileRef {
        let file = File {
            parent: WeakDirRef(Rc::downgrade(&self.0)),
            name,
            size,
        };
        let file_ref = FileRef(Rc::new(RefCell::new(file)));
        //have to unpack and repack to make a copy.
        self.0
            .borrow_mut()
            .children
            .push(Node::File(FileRef(Rc::clone(&file_ref.0))));
        file_ref
    }
}

impl File {
    fn get_parent(&self) -> &WeakDirRef {
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

impl Node {
    fn get_dir(&self) -> Option<DirRef> {
        match self {
            Node::Dir(dir) => Some(DirRef(Rc::clone(&dir.0))),
            Node::File(_) => None,
        }
    }
}

//transparent
impl Debug for DirRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("DirRef").field(self.0.borrow().deref()).finish()
        let this = self.0.borrow();
        write!(f, "{:?}", this);
        Ok(())
    }
}

//transparent
impl Debug for FileRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("FileRef").field(self.0.borrow().deref()).finish()
        let this = self.0.borrow();
        write!(f, "{:?}", this);
        Ok(())
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("File")
        //     .field("name", &self.name)
        //     .field("size", &self.size)
        //     .finish()
        write!(f, "- {} (file, size = {})", self.name, self.size);
        Ok(())
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self {
        //     Self::Dir(arg0) => f.debug_tuple("Dir").field(arg0).finish(),
        //     Self::File(arg0) => f.debug_tuple("File").field(arg0).finish(),
        // }
        match self {
            Node::Dir(dir) => write!(f, "{:?}", dir),
            Node::File(file) => write!(f, "{:?}", file),
        }
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Dir")
        //     .field("name", &self.name)
        //     .field("children", &self.children)
        //     .finish()
        writeln!(f, "- {} (dir)", self.name);
        for node in &self.children {
            //padding
            for (index, line) in format!("{:#?}", node).lines().enumerate() {
                writeln!(f, "  {line}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_tree_sizes() {
        let root = DirRef::new("/".to_string());
        let a = root.add(Node::Dir(DirRef::new("a".to_string())));
        let e = a
            .get_dir()
            .unwrap()
            .add(Node::Dir(DirRef::new("e".to_string())));
        let i = e.get_dir().unwrap().new_file("i".to_string(), 584);
        let f = a.get_dir().unwrap().new_file("f".to_string(), 29116);
        let g = a.get_dir().unwrap().new_file("g".to_string(), 2557);
        let h = a.get_dir().unwrap().new_file("h.lst".to_string(), 62596);
        let b = root.new_file("b.txt".to_string(), 14848514);
        let c = root.new_file("c.txt".to_string(), 8504156);
        let d = root.add(Node::Dir(DirRef::new("a".to_string())));
        let j = d.get_dir().unwrap().new_file("j".to_string(), 4060174);
        let dlog = d.get_dir().unwrap().new_file("d.log".to_string(), 8033020);
        let dext = d.get_dir().unwrap().new_file("d.ext".to_string(), 5626152);
        let k = d.get_dir().unwrap().new_file("k".to_string(), 7214296);
        println!("{:#?}", root);
        assert_eq!(e.get_dir().unwrap().total_size(), 584);
        assert_eq!(a.get_dir().unwrap().total_size(), 94853);
        assert_eq!(d.get_dir().unwrap().total_size(), 24933642);
        assert_eq!(root.total_size(), 48381165)
    }
}
