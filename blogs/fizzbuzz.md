# FizzBuzz

What is there to say about the FizzBuzz problem? It's a good place to begin with
a new language because it requires you to do some ordinary things, like create a range of numbers,
execute loops, make variable, use `if/else` control flow, and cast a number to a string.

## Typescript

Code: [fizzbuzz.ts](../typescript/src/fizzbuzz.ts)

I used inline lambda functions to help return determine the booleans in my `if/else` conditions.
This gains a little bit in readability over direct use of `i % n == 0`, but the same thing could
probably be better achieved as local shared variables inside the `for` block.  I chose to use back
tick formatting strings to convert the number to a string. I chose to have my fizzbuzz function
return an `Array`, and in order to print the values I mapped them over `console.log`. This is
also probably pretty unnecessary, I could've just console logged from directly in the loop for
this problem.

## Rust

Code: 

    - [lib](../rust/fizzbuzz/src/lib.rs)
    - [main](../rust/fizzbuzz/src/main.rs)

I made a lib function named `fizzbuzz` which returned a `Vec<String>` with the values between
`1` and `101` converted to `Fizz`, `Buzz`, and `FizzBuzz` as required. I ran into a lot of compiler
errors doing things I didn't understand along the way. I have no idea if this is a good way to do
this in rust, but at this point just getting a program to do something is a win. I considered using
a `match` to do the control flow, but I don't _really_ know how to do that. Perhaps as I progress
with rust I can come back and try this again.

I'm not sure `Vec` is the best choice here. This problem states a known number of 100
items in the vector returned, so maybe a fixed size `array` would have been appropriate.

## Conclusion

Done. Next!


