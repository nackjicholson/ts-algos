pub fn insertion_sort<T>(xs: &mut [T])
    where T: Ord
{
    let len = xs.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && xs[j] < xs[j - 1] {
            xs.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_numbers() {
        let mut numbers = vec![4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4];
    
        insertion_sort(&mut numbers);

        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_sorts_strings() {
        let mut strs = ["foo", "bar", "baz", "qux", "quux", "quuz", "grault", "corge"];
        let expected = ["bar", "baz", "corge", "foo", "grault", "quux", "quuz", "qux"];

        insertion_sort(&mut strs);
        
        assert_eq!(strs, expected);
    }
}
