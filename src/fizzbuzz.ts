/**
 * Write a program that prints the integers from 1 to 100 (inclusive).
 * But:
 *  - for multiples of three, print Fizz (instead of the number).
 *  - for mulitples of five, print Buzz (instead of the number).
 *  - for multiples of both three and five, print FizzBuzz (instead of the number).
 */

/*
 * IN:
 *  - No data in
 *
 * Out:
 *  ['1', '2', 'Fizz', '4', 'Buzz', 'Fizz', ..., '14', 'FizzBuzz', ...]
 */

function fizzBuzz(): string[] {
  let results = [];

  const isFizz = (i: number) => i % 3 === 0;
  const isBuzz = (i: number) => i % 5 === 0;
  const isFizzBuzz = (i: number) => isFizz(i) && isBuzz(i);

  for (let i = 1; i < 101; i++) {
    if (isFizzBuzz(i)) {
      results.push('FizzBuzz');
    } else if (isFizz(i)) {
      results.push('Fizz');
    } else if (isBuzz(i)) {
      results.push('Buzz');
    } else {
      results.push(`${i}`);
    }
  }
  return results;
}

fizzBuzz().map(a => console.log(a));
