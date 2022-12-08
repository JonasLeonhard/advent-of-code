mod parser;
mod tree;
use parser::{parse_commands, Command, TreeStruct};
use std::cell::RefCell;
use std::rc::Rc;
use tree::Node;

pub struct FileTree<'a> {
    root: Rc<RefCell<Node<TreeStruct<'a>>>>,
}

pub fn create_file_tree(input: &str) -> FileTree {
    let (_, commands): (&str, Vec<Command>) = parse_commands(input).unwrap();

    let file_tree = FileTree {
        root: Rc::new(RefCell::new(Node::new(TreeStruct::Dir("/"), None))),
    };
    let mut current_dir_node = file_tree.root.clone();

    for command in commands.iter() {
        match command {
            Command::Ls(tree_structs) => {
                for tree_struct in tree_structs {
                    current_dir_node
                        .borrow_mut()
                        .add_child(tree_struct.clone(), Some(current_dir_node.clone()));
                }
            }
            Command::Cd(dir_name) => match *dir_name {
                ".." => {
                    let parent_node = current_dir_node
                        .borrow_mut()
                        .parent_node
                        .as_ref()
                        .unwrap()
                        .clone();

                    current_dir_node = parent_node;
                }
                "/" => {
                    current_dir_node = file_tree.root.clone();
                }
                _ => {
                    let added_node = current_dir_node
                        .borrow_mut()
                        .add_child(TreeStruct::Dir(dir_name), Some(current_dir_node.clone()));

                    current_dir_node = added_node;
                }
            },
        };
    }

    file_tree
}

// recursivly gets the sum of the file-size size of the node and its children.
pub fn get_size(node: Rc<RefCell<Node<TreeStruct>>>) -> u32 {
    let node = node.borrow_mut();
    match node.content {
        TreeStruct::File((file_size, _file_name)) => file_size,
        TreeStruct::Dir(_dir) => node
            .clone()
            .children
            .into_iter()
            .fold(0, |acc, child_node| acc + get_size(child_node)),
    }
}

// recursivly gets the node and all its children
pub fn get_all_nodes(
    node: Rc<RefCell<Node<TreeStruct>>>,
    callback: &mut impl FnMut(&Rc<RefCell<Node<TreeStruct>>>),
) {
    callback(&node);

    let children = node.borrow_mut().children.clone();
    for child in children {
        get_all_nodes(child, callback);
    }
}

pub fn process_input1(file: String) -> u32 {
    let file_tree = create_file_tree(&file);

    let mut sum = 0;
    get_all_nodes(file_tree.root, &mut |node| {
        let current_node = node.clone();
        let is_dir = matches!(current_node.borrow_mut().content, TreeStruct::Dir(_));
        let node_size = get_size(current_node);

        if is_dir && node_size <= 100_000 {
            sum += node_size;
        }
    });

    sum
}

pub fn process_input2(file: String) -> u32 {
    let file_tree = create_file_tree(&file);

    const TOTAL_SIZE: u32 = 70000000;
    const NEEDED_SPACE: u32 = 30000000;
    let free_space = TOTAL_SIZE - get_size(file_tree.root.clone());
    let required_space = NEEDED_SPACE - free_space;
    let mut delete_candidate = u32::MAX;

    get_all_nodes(file_tree.root, &mut |node| {
        let current_node = node.clone();
        let is_dir = matches!(current_node.borrow_mut().content, TreeStruct::Dir(_));
        let node_size = get_size(current_node);

        if is_dir && node_size > required_space && node_size < delete_candidate {
            delete_candidate = node_size;
        }
    });

    delete_candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string()), 95437);
    }

    #[test]
    fn part_2() {
        let file = include_str!("test.txt");
        assert_eq!(process_input2(file.to_string()), 24933642);
    }
}
