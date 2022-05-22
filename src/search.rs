use std::usize;

use crate::sort;
// use crate::heap;


fn quick_select<T: Ord+Copy>(mut arr: &mut [T], mut index: usize) -> T {
    let mut mid = sort::part(arr, 0);

    while mid != index-1 {
        if mid < index-1 {
            arr = &mut arr[mid+1..];
            index = index-mid-1;
            mid = sort::part(arr, 0);
        } else {
            arr = &mut arr[..mid];
            mid = sort::part(arr, 0);
        }
    }
    arr[mid]
}

fn binary_search<T: Ord+Copy>(arr: &[T], valew: T) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();
    loop {
        if start==end { return None;}
        let mid = (start+end)/2;
        if arr[mid] == valew { 
            return Some(mid);
        } else if arr[mid] < valew {
            start = mid+1;
        } else {
            end = mid;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quick_select() {
        let mut arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        let mut ans = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        ans.sort();

        for i in 0..arr.len() {
            let value = quick_select(&mut arr, i+1);
            // print!("{} ", value);
            assert_eq!(value, ans[i]);
        }
        // println!("");
    }

    #[test]
    fn test_binary_search() {
        let mut arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        arr.sort();

        for i in 0..arr.len() {
            let temp = binary_search(&arr, arr[i]);
            assert_eq!(temp.unwrap(), i);
        }
        let temp = binary_search(&arr, 101010);
        assert!(temp.is_none());
        let temp = binary_search(&arr, 3);
        assert!(temp.is_none());
        let temp = binary_search(&arr, 11);
        assert!(temp.is_none());
        let temp = binary_search(&arr, 15);
        assert!(temp.is_none());
        let temp = binary_search(&arr, 20);
        assert!(temp.is_none());
        let temp = binary_search(&arr, 21);
        assert!(temp.is_none());
    }
}
