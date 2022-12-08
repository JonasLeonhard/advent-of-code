use std::cell::RefCell;
use std::rc::Rc;

type NodeHandle<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    pub content: T,
    pub children: Vec<NodeHandle<T>>,
    pub parent_node: Option<NodeHandle<T>>,
}

impl<T> Node<T> {
    pub fn new(content: T, parent_node: Option<NodeHandle<T>>) -> Self {
        Self {
            content,
            children: vec![],
            parent_node,
        }
    }

    pub fn add_child(
        &mut self,
        content: T,
        parent_node: Option<NodeHandle<T>>,
    ) -> Rc<RefCell<Node<T>>> {
        let pushed_ob = Rc::new(RefCell::new(Self::new(content, parent_node)));
        let pushed_ob_ref = pushed_ob.clone();
        self.children.push(pushed_ob);

        pushed_ob_ref
    }

    /// recursivly walk the contents of this node, and all children nodes
    pub fn walk(&self, f: &mut impl FnMut(&T)) {
        f(&self.content);

        for child in self.children.iter() {
            child.borrow().walk(f);
        }
    }
}
