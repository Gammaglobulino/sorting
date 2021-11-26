use std::fmt::Debug;

pub fn swap<A>(a1: &mut A, b1: &mut A)
where
    A: Copy,
{
    let hold = *a1;
    *a1 = *b1;
    *b1 = hold;
}
pub fn bubble_it<T>(v: &mut [T])
where
    T: Copy + PartialOrd + Debug,
{
    for _ in 0..v.len() - 1 {
        let mut it = v.iter_mut().peekable();
        while let Some(a) = it.next() {
            if let Some(next) = it.peek() {
                if *a > **next {
                    let hold = *a;
                    *a = **next;
                    if let Some(next) = it.next() {
                        *next = hold;
                    }
                }
            }
        }
        println!("{:?}", v);
    }
}

pub fn merge_it<T>(mut v: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Debug + Copy,
{
    if v.len() <= 1 {
        return v;
    }
    let right_half = v.split_off(v.len() / 2);
    let left_half = merge_it(v);
    let right_half = merge_it(right_half);
    let res=merge_two_arrays(left_half.to_vec(), right_half.to_vec());
    return res;
}

pub fn merge_two_arrays<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Debug + Copy,
{
    let mut res = Vec::with_capacity(a.len() + b.len());

    let mut b_it = b.into_iter().peekable();
    let mut a_it = a.into_iter().peekable();

    loop {
        if let Some(left) = a_it.peek() {
            if let Some(right) = b_it.peek() {
                if *left < *right {
                    res.push(*left);
                    a_it.next();
                } else {
                    res.push(*right);
                    b_it.next();
                    if let None = b_it.peek() {}
                }
            } else {
                res.extend(a_it);
                break;
            }
        } else {
            res.extend(b_it);
            break;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use crate::bubble_it;
    use crate::merge_it;
    use crate::merge_two_arrays;

    #[test]
    fn merge_linear_test() {
        let dest_vec = vec![7, 3, 2, 1, 5, 6, 4, 8];
        let res=merge_it(dest_vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], res);
    }
    #[test]
    fn merge_allthesame_test() {
        let dest_vec = vec![1, 1, 1, 1, 1,1,1,1];
        let res=merge_it(dest_vec);
        assert_eq!(vec![1, 1, 1, 1, 1, 1, 1, 1], res);
    }

    #[test]
    fn merge_half_the_same_test() {
        let dest_vec = vec![1, 1, 1, 1,2,2,2,2];
        let res=merge_it(dest_vec);
        assert_eq!(vec![1, 1, 1, 1, 2, 2, 2, 2], res);
    }

    #[test]
    fn merge_half_the_same_reversed_test() {
        let dest_vec = vec![2, 2, 2, 2,1,1,1,1];
        let res=merge_it(dest_vec);
        assert_eq!(vec![1, 1, 1, 1, 2, 2, 2, 2], res);
    }

    #[test]
    fn sorted_linear() {
        let mut dest_vec = [7, 3, 2, 1, 5, 6, 4, 8];
        bubble_it(&mut dest_vec);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], dest_vec);
    }
    #[test]
    fn already_sorted_linear() {
        let mut dest_vec = [1, 2, 3, 4, 5];
        bubble_it(&mut dest_vec);
        assert_eq!([1, 2, 3, 4, 5], dest_vec);
    }
    #[test]
    fn sorted_all_the_same() {
        let mut dest_vec = [1, 1, 1, 1];
        bubble_it(&mut dest_vec);
        assert_eq!([1, 1, 1, 1], dest_vec);
    }

    #[test]
    fn merge_two_arrays_test() {
        let mut origin = vec![7, 3, 2, 1, 5, 6, 4, 8];
        let b = &origin.split_off(origin.len() / 2);
        let a = &origin;

        let mut res = Vec::with_capacity(origin.len());

        let mut b_it = b.into_iter().peekable();
        let mut a_it = a.into_iter().peekable();

        assert_eq!(vec![5, 6, 4, 8], *b);
        assert_eq!(vec![7, 3, 2, 1], *a);

        loop {
            if let Some(left) = a_it.peek() {
                if let Some(right) = b_it.peek() {
                    if **left < **right {
                        res.push(**left);
                        a_it.next();
                    } else {
                        res.push(**right);
                        b_it.next();
                        if let None = b_it.peek() {}
                    }
                } else {
                    res.extend(a_it);
                    break;
                }
            } else {
                res.extend(b_it);
                break;
            }
        }

        assert_eq!(vec![5, 6, 4, 7, 3, 2, 1, 8], res);
    }

    #[test]
    fn merge_with_func_test() {
        let mut origin = vec![7, 3, 2, 1, 5, 6, 4, 8];
        let b = &origin.split_off(origin.len() / 2);
        let a = &origin;
        let res=merge_two_arrays(a.to_vec(),b.to_vec());
        
        assert_eq!(vec![5, 6, 4, 7, 3, 2, 1, 8], res);
    }
}
