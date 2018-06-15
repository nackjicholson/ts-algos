# Sort Arrays

Things in a list have to be sorted sometimes.

## Typescript

This code was pretty straight forward, and only in a few places actually took advantage of
typescript. Having sorting be a mutable process is a bit strange, but as it turns out rust sort is
a mutable operation as well. Using using and becoming aware of `ReadonlyArray` wasa nice
side effect of this exercise.

## Rust

Wow, I don't know how to program in rust! But, that's not really surprising. I had to look up and
explore the documentation a lot in order to port this code. I got to play with a `struct` for the
movie sorting. I had to use closures, `Ord`, `array`, `Vec`, `mut`, and reference passing. It was
a pretty basic task, but basic tasks are a stretch right now for me in rust.

## Conclusion

Good warmup exercise to get the feel for how to do very simple things in a new
language. The compiler errors were really helpful in both languages when I found
myself off-track.
