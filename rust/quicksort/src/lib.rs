use std::cmp::Ordering;

pub fn quicksort<T>(xs: &mut [T])
    where T: Ord
{
    let len = xs.len();
    quicksort_by(xs, 0, len, &|a, b| a.cmp(b));
}

pub fn quicksort_by<T, F>(xs: &mut [T], left: usize, right: usize, cmp: &F)
    where T: Ord,
          F: Fn(&T, &T) -> Ordering
{
    let len = right - left;
    if len <= 1 {
        return;
    }

    // I had a nasty bug here where I wasn't offsetting with left +
    // The result was 
    // [1,2,4,3] => [1, 4, 2, 3] instead of [1, 2, 3, 4]
    xs.swap(left, left + (len / 2));

    let mut pivot_rank = left;
    for i in (left + 1)..right {
        if cmp(&xs[i], &xs[left]) == Ordering::Less {
            pivot_rank += 1;
            xs.swap(i, pivot_rank);
        }
    }

    if pivot_rank != left {
        xs.swap(left, pivot_rank);
    }

    quicksort_by(xs, left, pivot_rank, cmp);
    quicksort_by(xs, pivot_rank + 1, right, cmp);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_numbers() {
        let mut numbers = vec![4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4];
    
        quicksort(&mut numbers);

        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_sorts_strings() {
        let mut strs = ["foo", "bar", "baz", "qux", "quux", "quuz", "grault", "corge"];
        let expected = ["bar", "baz", "corge", "foo", "grault", "quux", "quuz", "qux"];

        quicksort(&mut strs);
        
        assert_eq!(strs, expected);
    }
}
