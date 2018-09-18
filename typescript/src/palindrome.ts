import * as assert from 'assert';

/**
 *  * @module Palindrome solvers
 *   * A palindrome is a string that reads the same forward and backward, for example,
 *    * - radar, toot, madam.
 *     */

/**
 *  * Returns true if the string is a palindrome
 *   */
function isPalindrome(str: string): boolean {
  let s: string = str.trim().replace(/\s/g, '').toLowerCase();
  let p: string = s.split('').reverse().join('');
  return s === p;
}

/**
 * Returns true if ANY permutation of the string is a palindrome
 * civic true
 * vicic true
 * toot true
 * toto true
 * civil false
 */
function isAnyPermutationPalindrome(str: string): boolean {
    const unmatched = new Set<string>();
    str.trim().replace(/\s/g, '').toLowerCase().split('').forEach(char => {
      if (unmatched.has(char)) unmatched.delete(char);
      else unmatched.add(char);
    });
    return unmatched.size <= 1;
}

assert(isPalindrome('Racecar'));
assert(isPalindrome('A man a plan a canal Panama'));
assert(!isPalindrome('foo'));

assert(isAnyPermutationPalindrome('Racecar'));
assert(isAnyPermutationPalindrome('A man a plan a canal Panama'));
assert(isAnyPermutationPalindrome('foo'));
assert(isAnyPermutationPalindrome('civic'));
assert(isAnyPermutationPalindrome('vicic'));
assert(isAnyPermutationPalindrome('toot'));
assert(isAnyPermutationPalindrome('toto'));
assert(!isAnyPermutationPalindrome('civil'));
