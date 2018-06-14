/**
 * Sorting
 */

const arr: ReadonlyArray<string> = ['foo', 'bar'];
const copy = arr.slice().sort();
console.log({arr, copy});

const foo = [1, 3, 22];
foo.sort((a, b) =>
  /**
   * if a<b return -ve number
   * if a===b return 0
   * if a>b return +ve number
   */
  a - b
);
console.log(foo);

const movies = [
  {
    name: 'The Shawshank Redemption',
    year: 1994,
  },
  {
    name: 'The Godfather',
    year: 1972,
  },
  {
    name: 'The Godfather: Part II',
    year: 1974,
  },
  {
    name: 'The Dark Knight',
    year: 2008,
  },
];

const sortedMovies = movies.slice().sort((a, b) => a.year - b.year);
console.log({movies, sortedMovies});

const fooStrings = [
  'Alpha',
  'beta',
  'Gamma',
  'gramma',
  'delta',
];
fooStrings.sort((a, b) => a.localeCompare(b));
console.log(fooStrings);
