use std::usize;


fn prefix_arr(pattern: &str) -> Vec<usize> {
    let mut res = Vec::new();
    let p = pattern.as_bytes();
    for i in 0..p.len() {
        let mut ans = 0;
        let mut maybe_pos = i;
        while maybe_pos != 0 {
            maybe_pos = res[maybe_pos-1];
            if p[i] == p[maybe_pos] {
                ans = maybe_pos+1;
                break;
            }
        }
        res.push(ans);
    }
    res
}

fn kmp(source: &str, pattern: &str) -> Vec<usize>{
    let mut res = Vec::new();
    let arr = prefix_arr(pattern);
    let s = source.as_bytes();
    let p = pattern.as_bytes();
    let mut source_pointer = 0;
    let mut match_len = 0;
    while source_pointer != s.len() {
        // println!("{}", match_len);
        if s[source_pointer] == p[match_len] {
            source_pointer += 1;
            match_len += 1;
            if match_len==p.len() {
                res.push(source_pointer-p.len());
                match_len = arr[match_len-1];
            }
        } else {
            if match_len==0 {
                source_pointer += 1;
            } else {
                match_len = arr[match_len-1];
            }
            
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prefix_arr() {
        let source  = "abababababa";
        let pattern = "ababa";
        println!("{:?}", kmp(source, pattern));
    }
    #[test]
    fn each_letter_matches() {
        let index = kmp("aaa", "a");
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn a_few_separate_matches() {
        let index = kmp("abababa", "ab");
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn one_match() {
        let index =
            kmp("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn lots_of_matches() {
        let index = kmp("aaabaabaaaaa", "aa");
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn lots_of_intricate_matches() {
        let index = kmp("ababababa", "aba");
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn not_found0() {
        let index = kmp("abcde", "f");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found1() {
        let index = kmp("abcde", "ac");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found2() {
        let index = kmp("ababab", "bababa");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn empty_string() {
        let index = kmp("", "abcdef");
        assert_eq!(index, vec![]);
    }
}
