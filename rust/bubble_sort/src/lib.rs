pub fn bubble<T>(xs: &mut [T])
    where T: Ord
{
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
    fn it_bubble_sorts() {
        let mut numbers = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

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
