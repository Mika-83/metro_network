#[warn(dead_code)]
fn add_to_each(n: i32, lst: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let res = match lst.len() {
        0 => Vec::<Vec<i32>>::new(),
        _ => {
            let mut first = vec![n];
            first.append(&mut lst[0].to_vec());
            let mut rest = add_to_each(n, lst[1..].to_vec());
            let mut answer = vec![first];
            answer.append(&mut rest);
            answer
        },
    };
    res
}

#[warn(dead_code)]
fn prefix(lst: Vec<i32>) -> Vec<Vec<i32>> {
    let res = match lst.len() {
        0 => Vec::<Vec<i32>>::new(),
        _ => {
            let mut first = vec![vec![lst[0]]];
            let mut rest = add_to_each(lst[0], prefix(lst[1..].to_vec()));
            first.append(&mut rest);
            first
        },
    };
    res
}

// probpem 10.1
#[warn(dead_code)]
fn insert(n: i32, mut lst: Vec<i32>) -> Vec<i32> {
    let mut answer = Vec::<i32>::new();
    if lst.is_empty() {
        answer.push(n);
    } else if lst[0] >= n {
        answer.push(n);
        answer.append(&mut lst);
    } else if lst[lst.len() - 1] <= n {
        answer.append(&mut lst);
        answer.push(n);
    } else {
        answer.push(lst[0]);
        answer.append(&mut insert(n, lst[1..].to_vec()));
        }
    answer
}

// Problem 10.2
#[warn(dead_code)]
fn ins_sort(lst: Vec<i32>) -> Vec<i32> {
    let mut answer = Vec::<i32>::new();
    if !lst.is_empty() {
        answer.append(&mut insert(lst[lst.len() - 1], ins_sort(lst[..lst.len()-1].to_vec())));
    }
    answer
}

#[warn(dead_code)]
fn minimum(lst: Vec<i32>) -> i32 {
    let res = match lst.len() {
        0 => i32::MAX,
        _ => {
            let m = minimum(lst[1..].to_vec());
            if lst[0] <= m {
                lst[0]
            } else {
                m
            }
        }
    };
    res
}

#[warn(dead_code)]
fn append<T: Clone+Copy>(lst1: Vec<T>, lst2: Vec<T>) -> Vec<T> {
    let res = match lst1.len() {
        0 => lst2,
        _ => {
            let first = lst1[0];
            let rest = lst1[1..].to_vec();
            let mut ret = vec![first];
            ret.append(&mut append(rest, lst2));
            ret
        }
    };
    res
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn add_to_each_1() {
        assert_eq!(add_to_each(1i32,  Vec::<Vec<i32>>::new()), Vec::<Vec<i32>>::new())
    }
    #[test]
    fn add_to_each_2(){
        assert_eq!(add_to_each(1i32, vec![vec![2]]), vec![vec![1, 2]])
    }
    #[test]
    fn add_to_each_3() {
        assert_eq!(add_to_each(1i32, vec![vec![2], vec![2, 3], vec![2, 3, 4]]), vec![vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]])
    }
    #[test]
    fn prefix_1() {
        assert_eq!(prefix(Vec::<i32>::new()), Vec::<Vec<i32>>::new())
    }
    #[test]
    fn prefix_2() {
        assert_eq!(prefix(vec![1]), vec![vec![1]])
    }
    #[test]
    fn prefix_3() {
        assert_eq!(prefix(vec![1, 2, 3]), vec![vec![1], vec![1, 2], vec![1, 2, 3]])
    }
    #[test]
    fn insert_1() {
        assert_eq!(insert(1, vec![]), vec![1])
    }
    #[test]
    fn insert_2() {
        assert_eq!(insert(5, vec![1, 3, 4, 7, 8]), vec![1, 3, 4, 5, 7, 8])
    }
    #[test]
    fn ins_sort_1() {
        assert_eq!(ins_sort(vec![]), vec![])
    }
    #[test]
    fn ins_sort_2() {
        assert_eq!(ins_sort(vec![5, 3, 8, 1, 7, 4]), vec![1, 3, 4, 5, 7, 8])
    }
    #[test]
    fn ins_sort_3() {
        assert_eq!(ins_sort(vec![5, 5, 2, 1]), vec![1, 2, 5, 5])
    }
    #[test]
    fn ins_sort_4() {
        assert_eq!(ins_sort(vec![-1, -4, 4, 1]), vec![-4, -1, 1, 4])
    }
    #[test]
    fn minimum_1() {
        assert_eq!(minimum(Vec::<i32>::new()), i32::MAX)
    }
    #[test]
    fn minimum_2() {
        assert_eq!(minimum(vec![1, 3, 5]), 1)
    }
    #[test]
    fn minimum_3() {
        assert_eq!(minimum(vec![3, 2]), 2)
    }
    #[test]
    fn append_1(){
        assert_eq!(append(Vec::<i32>::new(), Vec::<i32>::new()), vec![])
    }
    #[test]
    fn append_2() {
        assert_eq!(append(vec![], vec![1, 2]), vec![1, 2])
    }
    #[test]
    fn append_3() {
        assert_eq!(append(vec![1, 2], vec![]), vec![1, 2])
    }
    #[test]
    fn append_4() {
        assert_eq!(append(vec![1, 2], vec![3, 4]), vec![1, 2, 3, 4])
    }
    #[test]
    fn append_5() {
        assert_eq!(append(vec!['a'], vec!['b']), vec!['a', 'b'])
    }
}