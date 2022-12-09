#[derive(Debug)]
struct Tree {
    height: u32,
}

struct Visibility {
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}

impl Visibility {
    fn is_visible_any(&self) -> bool {
        self.top || self.right || self.bottom || self.left
    }
}

#[derive(Debug)]
struct TreeGrid {
    grid: Vec<Vec<Tree>>,
}

impl TreeGrid {
    fn new(file: String) -> Self {
        let new_grid: Vec<Vec<Tree>> = file.lines().fold(vec![], |mut acc, line| {
            let trees = line
                .chars()
                .map(|char| {
                    let tree_height = char.to_digit(10).unwrap();
                    Tree {
                        height: tree_height,
                    }
                })
                .collect();
            acc.push(trees);
            acc
        });
        Self { grid: new_grid }
    }

    fn get_scenic_score(&self, line_index: usize, tree_index: usize) -> usize {
        let tree = &self.grid[line_index][tree_index];

        // iterates from row/column of the current index to the [Top].
        // Checks if all those cells are smaller trees. eg:
        // xXx (end)
        // xXx
        // xXx (<-) start from here, move vertically up until a higher tree is found
        // xxx
        let mut scenic_to_top = 0;
        for c_tree in self.grid[0..line_index]
            .iter()
            .rev()
            .map(|line| &line[tree_index])
        {
            scenic_to_top += 1;
            if c_tree.height >= tree.height {
                break;
            }
        }

        let mut scenic_to_bottom = 0;
        for c_tree in self.grid[line_index + 1..self.grid.len()]
            .iter()
            .map(|line| &line[tree_index])
        {
            scenic_to_bottom += 1;
            if c_tree.height >= tree.height {
                break;
            }
        }

        let mut scenic_to_left = 0;
        for c_tree in self.grid[line_index][0..tree_index].iter().rev() {
            scenic_to_left += 1;
            if c_tree.height >= tree.height {
                break;
            }
        }

        let mut scenic_to_right = 0;
        for c_tree in self.grid[line_index][tree_index + 1..self.grid.len()].iter() {
            scenic_to_right += 1;
            if c_tree.height >= tree.height {
                break;
            }
        }

        scenic_to_top * scenic_to_left * scenic_to_bottom * scenic_to_right
    }

    fn get_visibility(&self, line_index: usize, tree_index: usize) -> Visibility {
        let tree = &self.grid[line_index][tree_index];

        let is_outside = line_index == 0
            || line_index == self.grid.len() - 1
            || tree_index == 0
            || tree_index == self.grid[line_index].len() - 1;

        let mut visible_from_top = line_index == 0;
        let mut visible_from_bottom = line_index == self.grid.len() - 1;
        let mut visible_from_left = tree_index == 0;
        let mut visible_from_right = tree_index == self.grid[line_index].len() - 1;

        if !is_outside {
            // iterates from row/column of the current index to the [Top].
            // Checks if all those cells are smaller trees. eg:
            // xXx (<-) start from here, move vertically down to the line_index, tree_index node end.
            // xXx
            // xXx (end/search node)
            // xxx
            visible_from_top = self.grid[0..line_index]
                .iter()
                .map(|line| &line[tree_index])
                .all(|c_tree| c_tree.height < tree.height);

            visible_from_bottom = self.grid[line_index + 1..self.grid.len()]
                .iter()
                .map(|line| &line[tree_index])
                .all(|c_tree| c_tree.height < tree.height);

            visible_from_left = self.grid[line_index][0..tree_index]
                .iter()
                .all(|c_tree| c_tree.height < tree.height);

            visible_from_right = self.grid[line_index][tree_index + 1..self.grid.len()]
                .iter()
                .all(|c_tree| c_tree.height < tree.height);
        }

        Visibility {
            top: visible_from_top,
            right: visible_from_right,
            bottom: visible_from_bottom,
            left: visible_from_left,
        }
    }
}

pub fn process_input1(file: String) -> usize {
    let grid = TreeGrid::new(file);
    let mut visible_trees = vec![];

    for (col_index, col) in grid.grid.iter().enumerate() {
        for (row_index, tree) in col.iter().enumerate() {
            let tree_visibility = grid.get_visibility(col_index, row_index);
            if tree_visibility.is_visible_any() {
                visible_trees.push(tree);
            }
        }
    }

    visible_trees.len()
}

pub fn process_input2(file: String) -> usize {
    let grid = TreeGrid::new(file);
    let mut highest_scenic_score = 0;

    for (col_index, col) in grid.grid.iter().enumerate() {
        for (row_index, _tree) in col.iter().enumerate() {
            let scenic_score = grid.get_scenic_score(col_index, row_index);
            if highest_scenic_score < scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    highest_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string()), 21);
    }

    #[test]
    fn part_2() {
        let file = include_str!("test.txt");
        assert_eq!(process_input2(file.to_string()), 8);
    }
}
