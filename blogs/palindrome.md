# Palindrome

I rather like this problem for getting some introductory string manipulation. There are also
several interesting ways to solve a palindrome exercises. Extra complexity can be mixed in by
stating extra requirements around punctuation and case-sensitivity. You might have to strip
characters, use a regex, lowercase a string, reverse it, or run through a loop depending on how you
choose to solve it. The `is_any_permutation_palindrome` variation of this problem was not something
I had done before, and I definitely think it forced more thought about how to detect what a
palindrome is in a more clever way.

## Typescript

The `isPalindrome` solution I have here is how I initially would solve this problem. It's brute
force and simple.

1. Take your inbound string, trim it for whitespace, lowercase it.
2. Copy that trimmed, lowercase string and reverse it.
3. Are the two strings you made the same?

Nothing fancy here. This is how I as a human being think about what a palindrome is, and it's kind
of nice that the program I wrote works with my intuitive rules.

But, `isAnyPermutationPalindrome` challenges the intuitive rules a bit more. I just followed the
egghead.io video pretty much exactly and figured I'd solve it myself in more depth when I got to
rust.

## Rust

Again I did the same thing, I went with my intuitive solution for `is_palindrome`. In rust however
the same technique takes a little more thought about types. This is probably just because the
language and apis are less familiar to me. I ran into trouble trying to type my `s1` and `s2`
variables as `&str` and the compiler kept annoying me until I used `String`. I am not 100% sure
what the distinction is at this point, but I gather that I'm taking my inbound `&str` and turning
it into owned `String` objects which behave a lot like a `Vec`. The algorithm in rust is:

1. Take your inbound string slice, and lowercase all of it's chars, then turn them into an iterator
over the chars, and run a filter to get rid of all non-alphabetic chars. Collect the result into
a `String` collection.
2. Do that same thing again but this time use `rev` to reverse the `iterator`.
3. Are the `String` instances you produced the same?

It's a little bit more to deal with, but was a good tour of `String`, `&str`, and `char` methods.

The `is_any_permutation_palindrome` threw me off for a bit. I encounted a lot of compiler errors
trying to do this. I had the reference of what to do from the typescript solution but it was a bit
different to do in rust. Biggest challenges were realizing the `chars` method returns me owned
`chars` and I needed to pass a reference `&char` to the `HashSet::contains` and `HashSet::remove`
methods, but had to pass ownership back to my set when calling `HashSet::insert`. This does make
sense, but wow it's unusual for me to have to think about it like that in my home languages.

The way I think about this alogorithm is this; Imagine you have a collection of scrabble tiles and
a pail. If you want to know if those scrabble tiles can form a palindrome you go through the tiles
one at a time putting them in the pail. If you  come across a tile you already put in the pail,
reach in and take it out. When you're done going through your tiles, if there is only one tile in
the pail, you had a palindrome-able collection of tiles. That one leftover tile is the center
character of the palindrome and the others can fan out evenly on either side of it.


Cheers!
