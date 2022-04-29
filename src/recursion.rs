#[warn(dead_code)]
fn add_to_each(n: i32, mut lst: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
fn prefix(mut lst: Vec<i32>) -> Vec<Vec<i32>> {
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

#[cfg(test)]
mod tests {
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
}