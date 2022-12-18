use std::io::Write;

use itertools::Itertools;
use petgraph::{
    algo::{astar, dijkstra},
    dot::Dot,
    prelude::NodeIndex,
    Graph,
};

type Height = i32;
type Position = (isize, isize, char, Height);

/// Parse char to height. a == 1, ... z == 26. S equals a and E equals z.
fn parse_height(char: char) -> Height {
    (match char {
        'S' => 1,
        'E' => 26,
        _ => u32::from(char) - 96,
    } as i32)
}

fn parse_graph(file: String) -> Graph<Position, Height> {
    let mut graph: Graph<Position, Height> = Graph::new();
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    // fill the graph with position nodes from the grid
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // current position
            let node_height = parse_height(grid[y][x]);
            let position_coord = (x as isize, y as isize, grid[y][x], node_height);
            graph.add_node(position_coord);
        }
    }

    // create edges between cross_positions of nodes (UP, RIGHT, DOWN, LEFT)
    for node_index in graph.node_indices() {
        let node = graph[node_index];

        // get all cross_position nodes of the current node_index. Connect this node to its cross
        // node via add_edge.
        for cross_index in graph.node_indices() {
            let cross_node = graph[cross_index];
            let is_cross_node = cross_node.0 == node.0 - 1 && cross_node.1 == node.1 || // [x - 1][y]
                cross_node.0 == node.0 + 1 && cross_node.1 == node.1 || // [x + 1][y]
                cross_node.0 == node.0 && cross_node.1 == node.1 - 1 || // [x][y - 1]
                cross_node.0 == node.0 && cross_node.1 == node.1 + 1;

            if is_cross_node {
                let height_difference = cross_node.3 - node.3;

                // only <1 height difference is walkable
                if height_difference <= 1 {
                    graph.add_edge(node_index, cross_index, 1);
                }
            }
        }
    }

    graph
}

/// gets the length of the path from start_index to goal_index. If the goal_index is not connected
/// via an edge. It returns the largest height node, with the smallest path length
fn get_path_len_largest_height_smallest_path(
    graph: &Graph<Position, Height>,
    start_index: NodeIndex,
    goal_index: NodeIndex,
) -> (Position, i32) {
    let dijkstra_path = dijkstra(graph, start_index, Some(goal_index), |edge| *edge.weight());

    // get the largest height node, with the smalles path length
    let mut end_node: Option<Position> = None;
    let mut end_node_path_len = 0;

    for (node_index, path_len) in dijkstra_path.into_iter() {
        let node = graph[node_index];

        match end_node {
            Some(s_end_node) => {
                let node_height_equal = node.3 == s_end_node.3;
                let node_height_taller = node.3 > s_end_node.3;
                let node_path_smaller = path_len < end_node_path_len;

                if (node_height_equal || node_height_taller && node_path_smaller)
                    || node_index == goal_index
                {
                    end_node = Some(node);
                    end_node_path_len = path_len;

                    if node_index == goal_index {
                        break;
                    }
                }
            }
            None => {
                end_node = Some(node);
                end_node_path_len = path_len;
            }
        };
    }

    (end_node.unwrap(), end_node_path_len)
}

pub fn process_input1(file: String) -> i32 {
    let graph = parse_graph(file);

    let start_index = graph.node_indices().find(|i| graph[*i].2 == 'S').unwrap();
    let goal_index = graph.node_indices().find(|i| graph[*i].2 == 'E').unwrap();

    get_path_len_largest_height_smallest_path(&graph, start_index, goal_index).1
}

pub fn process_input2(file: String) -> i32 {
    let graph = parse_graph(file);

    let goal_index = graph.node_indices().find(|i| graph[*i].2 == 'E').unwrap();
    let start_indicies_tuple = graph
        .node_indices()
        .filter(|i| graph[*i].2 == 'S' || graph[*i].2 == 'a')
        .map(|start_index| {
            get_path_len_largest_height_smallest_path(&graph, start_index, goal_index)
        });

    let largest_heights = start_indicies_tuple
        .sorted_by_key(|node| node.0 .3)
        .collect_vec();

    let largest_height = *largest_heights.last().unwrap();
    let smallest_len_at_largest_height = largest_heights
        .iter()
        .filter(|position| position.0 .3 == largest_height.0 .3)
        .sorted_by_key(|position| position.1)
        .rev()
        .last()
        .unwrap(); // sort by length

    smallest_len_at_largest_height.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string()), 31);
    }

    #[test]
    fn part2() {
        let file = include_str!("test.txt");
        assert_eq!(process_input2(file.to_string()), 29);
    }
}
