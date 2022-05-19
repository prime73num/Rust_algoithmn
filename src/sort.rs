use std::cmp::Ordering;
fn part<T: Ord>(arr: &mut [T], idx: usize) -> usize {
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
    fn test_quick_sort() {
        let mut a = [1,5,3,33,7,4,7,88];
        let ans = [1,3,4,33,7,88,7,5];
        let idx = 5;
        let part = part(&mut a, idx);
        // println!("Part at {} : {:?}", part, a);
        assert_eq!(a, ans);
        quick_sort(&mut a);
        // println!("Sorted: {:?}", a);
        let ans = [1,3,4,5,7,7,33,88];
        assert_eq!(a, ans);
        let a = [1,2,2,2,2];
    }
    #[test]
    fn test_bubble() {
        let mut a = [8,5,3,22,7,4,10,88];
        // println!("{:?}", a);
        let ans = [3,4,5,7,8,10,22,88];
        bubble_sort(&mut a);
        // println!("{:?}", a);
        assert_eq!(a, ans);
    }
    #[test]
    fn test_insert() {
        let mut a = [8,5,3,22,7,4,10,88];
        // println!("{:?}", a);
        let ans = [3,4,5,7,8,10,22,88];
        insertion_sort(&mut a);
        // println!("{:?}", a);
        assert_eq!(a, ans);
    }
    #[test]
    fn test_merge() {
        let mut a = [8,5,3,22,7,4,10,88];
        // println!("{:?}", a);
        let ans = [3,4,5,7,8,10,22,88];
        merge_sort(&mut a);
        // println!("{:?}", a);
        assert_eq!(a, ans);
    }
}
