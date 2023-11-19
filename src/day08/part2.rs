#![allow(unused)]
use crate::{utilities::*, StructuredInput};

pub fn part2(input: &StructuredInput) -> usize {
    let mut max: usize = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let tree_score = tree_score(x, y, input);
            if max < tree_score{
                max = tree_score;
                println!("found new max with score {} at {}, {}", tree_score, x, y)
            }
        }
    }
    max
}
fn tree_score(x: usize, y: usize, input: &StructuredInput) -> usize {
    trees_visible_north(x, y, input)
        * trees_visible_east(x, y, input)
        * trees_visible_south(x, y, input)
        * trees_visible_west(x, y, input)
}
fn trees_visible_north(x: usize, y: usize, input: &StructuredInput) -> usize {
    if y == 0 {
        return 0;
    };
    let hight = input[y][x];
    let mut trees: usize = 0;
    for y in (0..y).rev() {
        trees += 1;
        if input[y][x] >= hight {
            break
        }
    }
    trees
}
fn trees_visible_east(x: usize, y: usize, input: &StructuredInput) -> usize {
    if x == input[0].len() {
        return 0;
    }
    let hight = input[y][x];
    let mut trees: usize = 0;
    for x in (x + 1)..input[0].len() {
        trees += 1;
        if input[y][x] >= hight {
            break
        }
    }
    trees
}
fn trees_visible_south(x: usize, y: usize, input: &StructuredInput) -> usize {
    if y == input.len() {
        return 0;
    };
    let hight = input[y][x];
    let mut trees: usize = 0;
    for y in (y + 1)..input.len() {
        trees += 1;
        if input[y][x] >= hight {
            break
        }
    }
    trees
}
fn trees_visible_west(x: usize, y: usize, input: &StructuredInput) -> usize {
    if x == input[0].len() {
        return 0;
    }
    let hight = input[y][x];
    let mut trees: usize = 0;
    for x in (0..x).rev() {
        trees += 1;
        if input[y][x] >= hight {
            break
        }
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(part2(&input), 8);
    }
    #[test]
    fn test_trees_visible() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(trees_visible_north(2, 3, &input), 2);
        assert_eq!(trees_visible_east(2, 3, &input), 2);
        assert_eq!(trees_visible_south(2, 3, &input), 1);
        assert_eq!(trees_visible_west(2, 3, &input), 2);
        assert_eq!(tree_score(2, 3, &input), 8);
    }
}
