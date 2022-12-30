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
    width: usize,
    height: usize,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn new(width: usize, height: usize, max_x: usize, max_y: usize) -> Self {
        Grid {
            grid: vec![vec![GridTile::Air; width]; height],
            width,
            height,
            max_x,
            max_y,
        }
    }

    fn get_relative_x(&self, x: usize) -> usize {
        self.width - ((self.max_x - x) + 1)
    }
    fn get_relative_y(&self, y: usize) -> usize {
        self.height - ((self.max_y - y) + 1)
    }

    /// display the grid in stdout
    fn display(&self) {
        // print!("\x1B[2J");
        for (y_index, tiles) in self.grid.iter().enumerate() {
            let three_digit_index = if y_index < 10 {
                "00".to_string() + y_index.to_string().as_str()
            } else if y_index < 100 {
                "0".to_string() + y_index.to_string().as_str()
            } else {
                y_index.to_string()
            };
            print!("{three_digit_index} | ");
            for tile in tiles {
                print!("{tile}");
            }
            println!()
        }
    }

    fn spawn_sand(&mut self, position: (usize, usize)) -> Option<(usize, usize)> {
        let tile = self.get_tile(position);
        if tile.is_some() {
            // self.grid[position.1][position.0] = GridTile::Sand;
            return Some((position.0, position.1));
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

        let current_tile = self.get_tile(sand_pos);
        let down_tile = self.get_tile(down);
        let left_tile = self.get_tile(left);
        let right_tile = self.get_tile(right);

        if let Some(down_tile) = down_tile {
            if down_tile.movable() {
                if *current_tile.unwrap() != GridTile::SandSource {
                    self.grid[sand_pos.1][sand_pos.0] = GridTile::Air;
                }
                self.grid[down.1][down.0] = GridTile::Sand;
                return Some((down.0, down.1));
            }
        }

        if let Some(left_tile) = left_tile {
            if left_tile.movable() {
                if *current_tile.unwrap() != GridTile::SandSource {
                    self.grid[sand_pos.1][sand_pos.0] = GridTile::Air;
                }
                self.grid[left.1][left.0] = GridTile::Sand;
                return Some((left.0, left.1));
            }
        }

        if let Some(right_tile) = right_tile {
            if right_tile.movable() {
                if *current_tile.unwrap() != GridTile::SandSource {
                    self.grid[sand_pos.1][sand_pos.0] = GridTile::Air;
                }
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
    let mut start_items: HashSet<(i32, i32, GridTile)> = rock_lines
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

    let sand_spawner = (500, 0, GridTile::SandSource);
    start_items.insert(sand_spawner.clone());

    // get the grid dimensions to fit all start_items inside
    let max_y = (start_items.iter().max_by_key(|item| item.1).unwrap().1) as usize;
    let min_y = (start_items.iter().min_by_key(|item| item.1).unwrap().1) as usize;
    let max_x = (start_items.iter().max_by_key(|item| item.0).unwrap().0) as usize;
    let min_x = (start_items.iter().min_by_key(|item| item.0).unwrap().0) as usize;
    let height = (max_y - min_y) + 1;
    let width = max_x - min_x;

    // add padding top-bottom for one extra space
    let padding_vertical = 2;

    // (https://www.triangle-calculator.com/?what=iso&a=C%3D45+h%3D500&submit=Solve)
    // 1. calculate a isoscles triangle with top degree of 45° and a height of cave.max_y:
    let isoscles_y_deg = 90_f64; // 45°
    let isoscles_height: f64 = max_y as f64 + ((padding_vertical as f64) / 2_f64) + 1_f64;
    // 2. calculate the angle α
    let isoscles_a_deg: f64 = 90_f64 - isoscles_y_deg / 2_f64;
    // 3. From the angle α and height h, we calculate side a:
    let isoscles_a_side_sin = isoscles_a_deg.to_radians().sin();
    let isoscles_a_side = isoscles_height / isoscles_a_side_sin;
    // 5.  From the side a and height h, we calculate side c - Pythagorean theorem:
    let isoscles_c_side = 2_f64 * (isoscles_a_side.powi(2) - isoscles_height.powi(2)).sqrt();

    let offset_of_sand_to_right = max_x as isize - sand_spawner.0 as isize; // Sand is at x=500;
    let offset_of_sand_to_left = min_x as isize - sand_spawner.0 as isize;
    let offset_of_sand_to_center = (offset_of_sand_to_left + offset_of_sand_to_right) / 2;

    let mut cave = Grid::new(
        isoscles_c_side as usize,
        height + padding_vertical,
        ((max_x + ((isoscles_c_side as usize - width) / 2)) as isize - offset_of_sand_to_center)
            as usize, // the max value has to be offset because the sand is not in the center
        max_y + padding_vertical / 2,
    );

    // fill the cave grid with starting positions
    for position in start_items {
        let rel_y = cave.get_relative_y(position.1 as usize);
        let rel_x = cave.get_relative_x(position.0 as usize);
        cave.grid[rel_y as usize][rel_x as usize] = position.2;
    }

    cave
}

pub fn process_input1(file: String) -> usize {
    let mut cave = parse_cave(file);
    cave.display();

    let sand_spawner = (cave.get_relative_x(500), cave.get_relative_y(0));
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
        cave.display();
    }

    sand_counter
}

pub fn process_input2(file: String) -> usize {
    let mut cave = parse_cave(file);
    cave.display();

    let sand_spawner = (cave.get_relative_x(500), cave.get_relative_y(0));
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

    // we return sand_counter + 1, as a sand will not be spawned inside the sand_spawner when all
    // three down positions of it are occupied.
    sand_counter + 1
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
