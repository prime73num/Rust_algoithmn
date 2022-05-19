
struct Node<T> {
   value: T,
   left: Link<T>,
   right: Link<T>
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
       Self {
           value, left: None, right: None
       } 
    }

}


type Link<T> = Option<Box<Node<T>>>;

struct Tree<T:Ord> {
    len: usize,
    root: Link<T>,
    // comparator: fn(&T, &T) -> bool,
}

enum Direction {
    Left,
    Right
}

impl<T:Ord> Tree<T> {
    fn new() -> Self {
        Self {
            len: 0,
            root: None,
        }
    }

    fn dfs(&self, rule: fn(&T)) {
        Tree::_dfs(&self.root, rule);
    }

    fn _dfs(root: &Link<T>, rule: fn(&T)) {
        if root.is_none() {return;}
        let node = root.as_ref().unwrap();
        rule(&node.value);
        Tree::_dfs(&node.left, rule);
        Tree::_dfs(&node.right, rule);
    }

    fn search(&self, value: T) -> Option<&Node<T>> {
        if self.root.is_none() {return None;}
        let mut root = self.root.as_ref().unwrap();
        if root.value == value { return Some(&*root);}
        let mut next = {
            if value < root.value {
                &root.left
            } else {
                &root.right
            }
        };
        while next.is_some() {
            root = next.as_ref().unwrap();
            if root.value == value { return Some(&*root);}
            next = {
                if value < root.value {
                    &root.left
                } else {
                    &root.right
                }
            };
        }
        None

    }
    fn add(&mut self, value: T) {
        if let None = self.root {
            self.root = Some(Box::new(Node::new(value)));
            return;
        }
        let mut root = self.root.as_mut().unwrap();
        let mut next = {
            if value < root.value {
                &mut root.left
            } else {
                &mut root.right
            }
        };
        while next.is_some() {
            root = next.as_mut().unwrap();
            next = {
                if value < root.value {
                    &mut root.left
                } else {
                    &mut root.right
                }
            };
        }
        *next = Some(Box::new(Node::new(value)));
    }
    fn is_leaf(node: &Node<T>) -> bool {
        match (&node.left, &node.right) {
            (None, None) => return true,
            _ => return false
        }
    }
    // fn direction(&self, root: &Node<T>, value: &T) -> Direction {
    //     match (self.comparator)(value, &root.value) {
    //         true => Direction::Left,
    //         false => Direction::Right
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use std::isize;

    use super::*;
    #[test]
    fn test_binary_tree() {
        let mut tt:Tree<isize> = Tree::new();
        tt.add(14);
        tt.add(12);
        tt.add(22);
        tt.add(9);
        tt.add(5);
        tt.add(1);
        tt.add(8);
        tt.add(5);
        tt.add(3);
        tt.add(17);
        tt.dfs(|x| {
            println!("{} ", x);
        });
        if let Some(x) = tt.search(117) {
            println!("Found {}", x.value);
        }
        let value = 3;
        match tt.search(value) {
            None => println!("Not found {}", value),
            Some(x) => println!("Found {}", x.value)
        }
    }
}
