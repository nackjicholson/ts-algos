/// # FizzBuzz
/// Write a program that prints the integers from 1 to 100 (inclusive).
/// But:
///  - for multiples of three, print Fizz (instead of the number).
///  - for mulitples of five, print Buzz (instead of the number).
///  - for multiples of both three and five, print FizzBuzz (instead of the number).

/*
 * IN:
 *  - No data in
 *
 * Out:
 *  ['1', '2', 'Fizz', '4', 'Buzz', 'Fizz', ..., '14', 'FizzBuzz', ...]
 */
pub fn fizzbuzz() -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    for n in 1..102 {
        if n % 15 == 0 {
            vec.push(String::from("FizzBuzz"));
        } else if n % 3 == 0 {
            vec.push(String::from("Fizz"));
        } else if n % 5 == 0 {
            vec.push(String::from("Buzz"));
        } else {
            vec.push(n.to_string());
        }
    }
    vec
}
