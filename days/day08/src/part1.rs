use crate::StructuredInput;

pub fn part1(input: &StructuredInput) -> usize {
    let mut acc: usize = 0;
    for y in 0..input.len(){
        for x in 0..input[0].len(){
            if is_visible(x, y, input){
                acc+=1;
            }
        }
    }
    acc
}

fn is_visible(x: usize, y: usize, input: &StructuredInput) -> bool {
    is_visible_north(x, y, input)
        || is_visible_east(x, y, input)
        || is_visible_south(x, y, input)
        || is_visible_west(x, y, input)
}
fn is_visible_north(x: usize, y: usize, input: &StructuredInput) -> bool {
    if y == 0 {
        return true;
    };
    let hight = input[y][x];
    for y in 0..y {
        if input[y][x] >= hight {
            return false;
        }
    }
    true
}
fn is_visible_east(x: usize, y: usize, input: &StructuredInput) -> bool {
    if x == input[0].len() {
        return true;
    }
    let hight = input[y][x];
    for x in (x + 1)..input[0].len() {
        if input[y][x] >= hight {
            return false;
        }
    }
    true
}
fn is_visible_south(x: usize, y: usize, input: &StructuredInput) -> bool {
    if y == input.len() {
        return true;
    };
    let hight = input[y][x];
    for y in (y + 1)..input.len() {
        if input[y][x] >= hight {
            return false;
        }
    }
    true
}
fn is_visible_west(x: usize, y: usize, input: &StructuredInput) -> bool {
    if x == input[0].len() {
        return true;
    }
    let hight = input[y][x];
    for x in 0..x {
        if input[y][x] >= hight {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(part1(&input), 21);
    }
}
