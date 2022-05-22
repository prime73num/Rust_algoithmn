use std::mem;


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

struct DFSIter<'a, T> {
    seq: Vec<&'a Node<T>>
}

impl<'a, T> Iterator for DFSIter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seq.is_empty() {
            None
        } else {
            self.seq.pop()
        }
    }
    
}

impl<'a, T:Ord> DFSIter<'a, T> {
    fn new(tree: &'a Tree<T>) -> DFSIter<'a, T> {
        let mut seq = Vec::new();
        DFSIter::_dfs(&tree.root, &mut seq);
        Self { seq }
    }
    fn _dfs<'b>(root: &'b Link<T>, seq: &mut Vec<&'b Node<T>>) {
        if root.is_none() {return;}
        let node = root.as_deref().unwrap();
        DFSIter::_dfs(&node.right, seq);
        seq.push(node);
        DFSIter::_dfs(&node.left, seq);
    }
}

impl<T: Ord+Copy> Tree<T> {
    fn construct(array: &[T]) -> Self {
        let mut res = Self::new();
        for i in array.iter() {
            res.add(*i);
        }
        res
    }
}

impl<T:Ord> Tree<T> {
    fn new() -> Self {
        Self {
            len: 0,
            root: None,
        }
    }
    fn iter_dfs(&self) -> DFSIter<T> {
        DFSIter::new(self)
    }
    fn inorder_big<'a>(root: &'a Node<T>) -> &'a Link<T> {
        if root.right.is_none() { return &root.right;}
        let mut root = &root.right;
        while root.as_ref().unwrap().left.is_some() {
            root = &root.as_ref().unwrap().left;
        }
        root
    }
    fn inorder_small<'a>(root: &'a Node<T>) -> &'a Link<T> {
        if root.left.is_none() { return & root.left;}
        let mut root = &root.left;
        while root.as_ref().unwrap().right.is_some() {
            root = &root.as_ref().unwrap().right;
        }
        root
    }

    fn inorder_big_mut<'a>(root: &'a mut Node<T>) -> &'a mut Link<T> {
        if root.right.is_none() { return &mut root.right;}
        let mut root = &mut root.right;
        while root.as_ref().unwrap().left.is_some() {
            root = &mut root.as_mut().unwrap().left;
        }
        root
    }
    fn inorder_small_mut<'a>(root: &'a mut Node<T>) -> &'a mut Link<T> {
        if root.left.is_none() { return &mut root.left;}
        let mut root = &mut root.left;
        while root.as_ref().unwrap().right.is_some() {
            root = &mut root.as_mut().unwrap().right;
        }
        root
    }

    #[allow(unused_must_use)]
    fn delete(&mut self, value: T) {
        let position = self.search_mut(&value);
        if position.is_none() {return;}
        if Tree::is_leaf(position.as_deref().unwrap()) {
            *position = None;
        } else {
            let root = Tree::inorder_small_mut(position.as_deref_mut().unwrap());
            if root.is_some() {
                let leftson = mem::replace(&mut root.as_deref_mut().unwrap().left, None);
                let parent = mem::replace(root, leftson);
                mem::replace(&mut position.as_deref_mut().unwrap().value, parent.unwrap().value);
                return;
            }
            let root = Tree::inorder_big_mut(position.as_deref_mut().unwrap());
            if root.is_some() {
                let rightson = mem::replace(&mut root.as_deref_mut().unwrap().right, None);
                let parent = mem::replace(root, rightson);
                mem::replace(&mut position.as_deref_mut().unwrap().value, parent.unwrap().value);
            }
        }
    }

    fn search_mut(&mut self, value: &T) -> &mut Link<T> {
        let mut this = &mut self.root;
        loop {
            if this.is_none() || this.as_deref().unwrap().value==*value {
                return this;
            }
            let root = this.as_deref_mut().unwrap();
            this = {
                if *value < root.value {
                    &mut root.left
                } else {
                    &mut root.right
                }
            };
        }
    }
    fn search(&self, value: &T) -> &Link<T> {
        let mut this = &self.root;
        loop {
            if this.is_none() || this.as_deref().unwrap().value==*value {
                return this;
            }
            let root = this.as_deref().unwrap();
            this = {
                if *value < root.value {
                    &root.left
                } else {
                    &root.right
                }
            };
        }
    }
    fn add(&mut self, value: T) {
        let mut this = &mut self.root;
        loop {
            if this.is_none() {
                *this = Some(Box::new(Node::new(value)));
                break;
            }
            let root = this.as_deref_mut().unwrap();
            this = {
                if value < root.value {
                    &mut root.left
                } else {
                    &mut root.right
                }
            };
        }
        self.len += 1;
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
    use super::*;

    #[test]
    fn test_add_iter() {
        let mut arr = [2,33,44,2,4,1,8,57,46,25,12,554,226,90,40,19];
        let mut ans:Vec<isize> = Vec::new();

        let tt = Tree::construct(&arr);
        let iter = tt.iter_dfs();

        for i in iter {
            ans.push(i.value);
            // print!("{} ", i.value);
        }
        // println!("");

        arr.sort();
        assert_eq!(arr.to_vec(), ans);

        // let ans = "1 3 5 5 8 9 12 14 17 22";
        // assert_eq!(res.as_str(), ans);
    }
    #[test]
    fn test_search() {
        let arr = [2,33,44,2,4,1,8,57,46,25,12,445,226,90,40,19];
        let bst = Tree::construct(&arr);
        for i in arr.iter() {
            let temp = bst.search(i).as_deref().unwrap();
            assert_eq!(temp.value, *i);
        }
        assert!(bst.search(&3333).is_none());
        assert!(bst.search(&10).is_none());
        assert!(bst.search(&45).is_none());
        assert!(bst.search(&11).is_none());
        assert!(bst.search(&58).is_none());
        assert!(bst.search(&39).is_none());
    }

    #[test]
    fn test_inorder() {
        let arr = [46,33,44,2,4,1,8,57,2,25,12,445,226,90,40,19];
        let bst = Tree::construct(&arr);
        let mut iter = bst.iter_dfs();
        let mut pre = iter.next().unwrap();
        for node in iter {
            let mut f = false;
            let mut s = false;
            match Tree::inorder_big(pre) {
                Some(x) => {
                    if x.value == node.value {
                        // println!("{} inorder big is {}", pre.value, node.value);
                        f = true;
                    }
                },
                None => {}
            }
            match Tree::inorder_small(node) {
                Some(x) => {
                    if x.value == pre.value {
                        s = true;
                        // println!("{} inorder small is {}", node.value, pre.value);
                    }
                },
                None => {}
            }
            pre = node;
            assert!(f != s);
        }
    }

    #[test]
    fn test_delete() {
        let arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        let mut bst = Tree::construct(&arr);
        for i in bst.iter_dfs() {
            print!("{} ", i.value);
        }
        println!("");

        for i in arr.iter() {
            // Require the binary tree does not have same value.
            assert!(bst.search(i).is_some());
            bst.delete(*i);
            assert!(bst.search(i).is_none());
        }
    }

    // #[test]
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
        // tt.dfs(|x| {
            // println!("{} ", x);
        // });
        if let Some(x) = tt.search(&117) {
            println!("Found {}", x.value);
        }
        let value = 3;
        match tt.search(&value) {
            None => println!("Not found {}", value),
            Some(x) => println!("Found {}", x.value)
        }
        match Tree::inorder_big(tt.root.as_deref_mut().unwrap()) {
            Some(x) => {
                println!("{}", x.value);
            },
            None => println!("Not found")
        }
        match Tree::inorder_small(tt.root.as_deref_mut().unwrap()) {
            Some(x) => {
                println!("{}", x.value);
            },
            None => println!("Not found")
        }
    }
}
