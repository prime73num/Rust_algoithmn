use std::cmp::Ordering;

pub fn part<T: Ord>(arr: &mut [T], idx: usize) -> usize {
    let mut i:isize = -1;
    let mut j = 0;
    arr.swap(idx, arr.len()-1);
    let idx = arr.len()-1;
    while j != arr.len() {
        match arr[j].cmp(&arr[idx]) {
            Ordering::Less => {
                i += 1;
                arr.swap(i as usize, j);
            },
            _ => {}
        }
        j += 1;
    }
    i += 1;
    arr.swap(i as usize, idx);
    i as usize
}

pub fn quick_sort<T: Ord>(arr:&mut [T]) {
    if arr.len() == 0 {return;}
    let idx = arr.len()-1;
    let mid = part(arr, idx);
    quick_sort(&mut arr[..mid]);
    quick_sort(&mut arr[mid+1..]);
    // let mid = part(arr, idx) as isize;
    // if mid > 0 {
    //     quick_sort(&mut arr[0..mid as usize]);
    // }
    // if (mid as usize) < arr.len()-1 {
    //     let start = (mid + 1) as usize;
    //     quick_sort(&mut arr[start..]);
    // }
}

fn bubble_sort<T:Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut temp = 0;
        while temp != arr.len()-i {
            if arr[temp] > arr[temp+1] {
                arr.swap(temp, temp+1);
            }
            temp += 1;
        }
    }
}

fn insertion_sort<T:Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        let mut temp = i+1;
        while (temp != 0) && arr[temp-1] > arr[temp] {
            arr.swap(temp, temp-1);
            temp -= 1;
        }
    } 
}

fn merge_sort<T:Ord+Copy>(arr:&mut [T]) {
    if arr.len()==1 {return;}
    let mid = arr.len()/2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();
    let mut l = 0;
    let mut r = 0;
    for v in arr {
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let mut arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        let mid = part(&mut arr, 6);
        for i in &arr[..mid] {
            assert!(*i<arr[mid]);
        }
        for i in &arr[mid..] {
            assert!(*i>=arr[mid]);
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        let mut ans = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        ans.sort();
        quick_sort(&mut arr);
        assert_eq!(arr, ans);
    }
    #[test]
    fn test_bubble() {
        let mut arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        let mut ans = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        ans.sort();
        bubble_sort(&mut arr);
        assert_eq!(arr, ans);
    }
    #[test]
    fn test_insert() {
        let mut arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        let mut ans = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        ans.sort();
        insertion_sort(&mut arr);
        assert_eq!(arr, ans);
    }
    #[test]
    fn test_merge() {
        let mut arr = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        let mut ans = [46,33,44,2,4,1,8,57,22,25,12,445,226,90,40,19];
        ans.sort();
        merge_sort(&mut arr);
        assert_eq!(arr, ans);
    }
}
