use itertools::{self, Itertools};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum GridTile {
    Rock,
    Sand,
    SandSource,
    Air,
}

impl GridTile {
    fn movable(&self) -> bool {
        match self {
            GridTile::Rock => false,
            GridTile::Sand => false,
            GridTile::SandSource => false,
            GridTile::Air => true,
        }
    }
}
impl Display for GridTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GridTile::Rock => write!(f, "#"),
            GridTile::Sand => write!(f, "o"),
            GridTile::SandSource => write!(f, "+"),
            GridTile::Air => write!(f, "."),
        }
    }
}
struct Grid {
    grid: Vec<Vec<GridTile>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            grid: vec![vec![GridTile::Air; width]; height],
        }
    }

    /// display the grid in stdout
    fn display(&self) {
        // print!("\x1B[2J");
        for (y_index, tiles) in self.grid.iter().enumerate() {
            print!("{y_index} | ");
            for tile in tiles {
                print!("{tile}");
            }
            println!()
        }
    }

    fn spawn_sand(&mut self, position: (usize, usize)) -> Option<(usize, usize)> {
        let tile = self.get_tile(position);
        if tile.is_some() {
            self.grid[position.1][position.0] = GridTile::Sand;
            return Some((position.0, position.1 + 1));
        }
        None
    }

    fn get_tile(&self, position: (usize, usize)) -> Option<&GridTile> {
        if let Some(row) = self.grid.get(position.1) {
            row.get(position.0)
        } else {
            None
        }
    }

    /// moves sand either down, left or right if the tile there is movable. Then returns if the
    /// sand has moved
    fn move_sand(&mut self, sand_pos: (usize, usize)) -> Option<(usize, usize)> {
        // position check to keep sand in bounds of grid
        if sand_pos.1 == self.grid.len() - 1
            || sand_pos.0 == self.grid[0].len() - 1
            || sand_pos.0 == 0
            || sand_pos.1 == 0
        {
            return None;
        }

        let down = (sand_pos.0, sand_pos.1 + 1);
        let left = (sand_pos.0 - 1, sand_pos.1 + 1);
        let right = (sand_pos.0 + 1, sand_pos.1 + 1);

        let down_tile = self.get_tile(down);
        let left_tile = self.get_tile(left);
        let right_tile = self.get_tile(right);

        if let Some(down_tile) = down_tile {
            if down_tile.movable() {
                self.grid[sand_pos.1][sand_pos.0] = GridTile::Air;
                self.grid[down.1][down.0] = GridTile::Sand;
                return Some((down.0, down.1));
            }
        }

        if let Some(left_tile) = left_tile {
            if left_tile.movable() {
                self.grid[sand_pos.1][sand_pos.0] = GridTile::Air;
                self.grid[left.1][left.0] = GridTile::Sand;
                return Some((left.0, left.1));
            }
        }

        if let Some(right_tile) = right_tile {
            if right_tile.movable() {
                self.grid[sand_pos.1][sand_pos.0] = GridTile::Air;
                self.grid[right.1][right.0] = GridTile::Sand;
                return Some((right.0, right.1));
            }
        }

        None
    }
}

fn parse_cave(file: String) -> Grid {
    let rock_lines: Vec<Vec<(i32, i32)>> = file
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|position| {
                    let (x, y) = position.split_once(',').unwrap();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect()
        })
        .collect();

    // create all positions for rock line edges. Lines can only be horizontal or vertical
    let start_items: HashSet<(i32, i32, GridTile)> = rock_lines
        .iter()
        .flat_map(|polyline| {
            polyline.iter().tuple_windows().flat_map(|(left, right)| {
                let line_is_horizontal = right.1 - left.1 == 0;

                let amount_of_grid_tiles = if line_is_horizontal {
                    right.0 - left.0
                } else {
                    right.1 - left.1
                };

                let range = if amount_of_grid_tiles < 0 {
                    amount_of_grid_tiles..=0
                } else {
                    0..=amount_of_grid_tiles
                };

                range.map(move |index| {
                    if line_is_horizontal {
                        (left.0 + index, left.1, GridTile::Rock)
                    } else {
                        (left.0, left.1 + index, GridTile::Rock)
                    }
                })
            })
        })
        .collect();

    // get the grid dimensions to fit all start_items inside
    let max_y = (start_items.iter().max_by_key(|item| item.1).unwrap().1) as isize;
    // let min_y = (start_items.iter().min_by_key(|item| item.1).unwrap().1) as isize;
    // let max_x = (start_items.iter().max_by_key(|item| item.0).unwrap().0) as isize;
    // let min_x = (start_items.iter().min_by_key(|item| item.0).unwrap().0) as isize;
    // let height = ((max_y - min_y) + 1) as isize;
    // let width = ((max_x - min_x) + 1) as isize;
    // let width = (width * 2) + 2;
    let width = 500 + max_y + 2;

    // let get_relative_x = |x: isize| width - ((max_x - x) + 1);
    // let get_relative_y = |y: isize| height - ((max_y - y) + 1);

    let mut cave = Grid::new(width as usize, max_y as usize + 2);

    // fill the cave grid with starting positions
    for position in start_items {
        // let rel_y = get_relative_y(position.1 as isize);
        // let rel_x = get_relative_x(position.0 as isize);
        cave.grid[position.1 as usize][position.0 as usize] = position.2;
    }

    cave
}

pub fn process_input1(file: String) -> usize {
    let mut cave = parse_cave(file);

    let sand_spawner = (500, 0);
    let sand = cave.spawn_sand(sand_spawner).unwrap();
    let mut sand: Option<(usize, usize)> = cave.move_sand(sand);
    let mut sand_counter = 0;

    while let Some(next_sand_pos) = sand {
        // is the next sand position bottom of the grid
        if next_sand_pos.1 == cave.grid.len() - 1 {
            break;
        }
        sand = cave.move_sand(next_sand_pos); // recursivly move again until sand cant fall further

        if sand.is_none() {
            let new_sand = cave.spawn_sand(sand_spawner).unwrap();
            sand_counter += 1;
            sand = cave.move_sand(new_sand);
        }
    }
    cave.display();
    sand_counter
}

pub fn process_input2(file: String) -> usize {
    let mut cave = parse_cave(file);

    let sand_spawner = (500, 0);
    let sand = cave.spawn_sand(sand_spawner).unwrap();
    let mut sand: Option<(usize, usize)> = cave.move_sand(sand);
    let mut sand_counter = 0;

    while let Some(next_sand_pos) = sand {
        // is the next sand position outside of the grid
        if next_sand_pos == sand_spawner {
            break;
        }
        sand = cave.move_sand(next_sand_pos); // recursivly move again until sand cant fall further

        if sand.is_none() {
            let new_sand = cave.spawn_sand(sand_spawner).unwrap();
            sand_counter += 1;
            sand = cave.move_sand(new_sand);
        }
    }
    cave.display();
    sand_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string()), 24);
    }

    #[test]
    fn part_02() {
        let file = include_str!("test.txt");
        assert_eq!(process_input2(file.to_string()), 93);
    }
}
