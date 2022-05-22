
pub struct Heap<T> {
    len: usize,
    item: Vec<T>,
    comparator: fn(&T, &T) -> bool
    // fn( small, big ) -> true
    // fn( big, small ) -> false
}

impl<T:Default> Heap<T> {
    pub fn new(comparator: fn(&T, &T)-> bool) -> Self {
        Self { len: 0, item: vec![T::default()], comparator } 
    }
    pub fn construct(comparator: fn(&T, &T)-> bool, arr: Vec<T>) -> Self {
        let mut temp = Self {
            len: arr.len(),
            item: arr,
            comparator
        };
        temp.item.push(T::default());
        temp.item.swap(0, temp.len);
        let end = (temp.len)/2+1;
        for i in {1..end}.rev() {
            temp.down(i);
        }
        temp
    }
    pub fn front(&self) -> &T {
        &self.item[1]
    }
    pub fn add(&mut self, valte: T) {
        self.item.push(valte);
        self.len += 1;
        self.up(self.item.len()-1);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let tail = self.item.len()-1;
        self.item.swap(1, tail);
        let res = self.item.pop().unwrap();
        self.len -= 1;
        self.down(1);
        Some(res)
    }
    pub fn is_empty(&self) -> bool {
        return self.len==0;
    }
    pub fn to_vec(mut self) -> Vec<T> {
        self.item.remove(0);
        self.item
    }
    fn has(&self, idx: usize) -> bool {
        return idx<self.item.len();
    }
    fn down(&mut self, mut idx: usize) {
        let mut next = self.direction(idx);
        while next != idx {
            self.item.swap(idx, next);
            idx = next;
            next = self.direction(idx);
        }
    }
    fn up(&mut self, mut idx: usize) {
        let mut next = {
            if idx==1 || (self.comparator)(&self.item[idx/2],&self.item[idx]) {
                idx
            } else {
                idx/2
            }
        };
        while next != idx {
            self.item.swap(idx, next);
            idx = next;
            next = {
                if idx==1 || (self.comparator)(&self.item[idx/2],&self.item[idx]) {
                    idx
                } else {
                    idx/2
                }
            };
        }
    }
    fn direction(&self, idx: usize) -> usize {
        let mut res = idx;
        let son = 2*idx;
        if self.has(son) {
            if (self.comparator)(&self.item[son],&self.item[idx]) {
                res = son;
            }
        } 
        let son = 2*idx+1;
        if self.has(son) {
            if (self.comparator)(&self.item[son],&self.item[res]) {
                res = son;
            }
        } 
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_output() {
        let mut heap:Heap<isize> = Heap::new(|x,y| {x<y});
        heap.add(2);
        heap.add(1);
        heap.add(433);
        heap.add(9);
        heap.add(4);
        heap.add(22);
        heap.add(19);
        let mut output:Vec<isize> = Vec::new();
        while let Some(v) = heap.pop() {
            output.push(v);
        }
        // println!("{:?}", output);

        let mut arr = [33,4,6,2,8,9,13,55,22,220,30,24,99,25,13,21,9];
        let input = arr.to_vec();
        let mut heap = Heap::construct(|x,y| {x<y}, input);
        let mut output:Vec<isize> = Vec::new();
        while let Some(v) = heap.pop() {
            output.push(v);
        }
        // println!("{:?}", output);
        arr.sort();
        assert_eq!(arr.to_vec(), output);

    }

    #[test]
    fn test_construct() {
        let arr = [46,33,44,2,4,1,8,57,22,25,12,445,226];
        let max_heap = Heap::construct(|x,y| {x>y}, arr.to_vec());
        let out = max_heap.to_vec();
        let ans = [445, 44, 226, 25, 33, 46, 57, 22, 4, 12, 1, 8, 2].to_vec();
        assert_eq!(out, ans);
    }
}
