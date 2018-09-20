# Bubble Sort

This algorithm is a very intuitive way to sort a list of values. The basic approach is to compare a first value in the list with a second value in the list, and if if the first value is larger -- swap their positions. The way this is accomplished in the conceptual example is a "double" iteration. For every item in the list, we iterate from that items position in the list to the end, swapping values. Displaying the array through each iteration makes it appear that a value "bubbles" through the list into it's proper position.

# Typescript

As always, the swapping of array positions via destructuring syntax is really confusing. Other than that this algorithm was pretty easy to grok. The holding of state in the `swapped` variable is a pretty simple way to cut the loop early if there is no more reordering to do. It's basically saying, if the previous run the through the `for` loop didn't find anything out of order, we can be certain we're done; don't try looking again.

# Rust

I've been busy with a real world python and typescript project and I haven't had much time to work with rust. It was a difficult reintroduction, but I actually had a lot of fun. The rust implementation taught me more about the algorithm, probably because the video series I'm following on egghead.io doesn't hold my hand when I go off track and do it in rust.

In rust I expanded the exercise to sort lists of any `Ord` type, rather than just numbers. This got me dipping my toes in generic type signatures and rummaging around the `Vec` docs. I briefly thought about the `Peekable` trait for inspecting the `i+1` item in the list, but ultimately decided that the clearest way was pretty much the same as the Typescript approach. I thought the `.swap` method was a much more familiar (pythonic) way to swap items in the vector. I encountered a lot of compiler errors along the way, but it pretty much drove me to the code I have. That's saying a lot, because early iterations really were kind of wacky. I think it will become a bit of a skill to know when the compiler is trying to tell me I'm doing something I shouldn't do and when it's being daft. I need to develop a fight or flight instinct in regard to `rustc`, and `tsc` for that matter.


