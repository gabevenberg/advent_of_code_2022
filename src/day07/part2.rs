use crate::file_tree::*;

const TOTAL_SPACE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;

pub fn part2(input: NodeRef) -> usize {
    let used_space = input.borrow().get_total_size();
    let unused_space = TOTAL_SPACE - used_space;
    let space_to_free = NEEDED_SPACE - unused_space;
    let dirs = input.get_all_dirs();
    dirs.iter()
        .map(|d| d.borrow().get_total_size())
        .filter(|s| *s >= space_to_free)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::{ops::Deref, rc::Rc};

    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!(part2(root), 24933642);
    }
}
