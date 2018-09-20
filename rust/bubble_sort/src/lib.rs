pub fn bubble_sort_concept(numbers: &Vec<usize>) -> Vec<usize> {
    let mut ns = numbers.clone();
    let len = numbers.len();

    for _ in 0..len {
        for i in 0..len-1 {
            if ns[i] > ns[i + 1] {
                ns.swap(i, i + 1);
            }
        }
    }

    ns
}

pub fn bubble<T: Ord>(xs: &mut Vec<T>) {
    let len = xs.len();

    loop {
        let mut swapped = false;
        for i in 0..(len - 1) {
            if xs[i] > xs[i + 1] {
                xs.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_bubble_concept() {
        let numbers = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let actual = bubble_sort_concept(&numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_bubble_sorts() {
        let mut numbers = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        bubble(&mut numbers);
        assert_eq!(&numbers, &expected)
    }

    #[test]
    fn it_bubble_sorts_strings() {
        let mut strs = vec!["foo", "bar", "baz", "qux", "quux", "quuz", "corge", "grault"];
        let expected = vec!["bar", "baz", "corge", "foo", "grault", "quux", "quuz", "qux"];

        bubble(&mut strs);
        assert_eq!(&strs, &expected);
    }
}
