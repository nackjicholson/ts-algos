# Anagram

Take a word, compare it to another word and see if you can make one word from the other. I
initially thought it sounded like a palindrome exercise, but it was a little more complicated. In
palindrome you just have to use some string methods to reverse a string and filter out white space.
This required iterating over a string as a collection of characters, and using a group by 
technique.

## Typescript

My initial brainstorm on how to do this went like this:

1. Go through each string finding index of each letter in the other string and make sure you
can find all chars in s1, in s2. No that's way to slow and difficult.
2. Try to do a group by and sum of the number of each char, do this for each string and make sure
the sums are all equal. Wasn't really sure how to best implement a groupBy without reaching
for `lodash` though.

The solution provided in the egghead video series is probably not what I would've come up with
on my own, but it's not much different than #2 above.  I did not think about keeping one object
and checking all values are zeroes after going through each loop. I also probably would not have
reached straight away for the `Map` type, favoring a plain object instead.

I also had some trouble getting my code to compile. I found that I couldn't import the `assert`
module, and for some reason `Map` didn't exist.

# Rust

I followed the same approach as in the typescript. I am not convinced it's the most rustic way to
accomplish this ticket, but it does the job. It was good experience to use the HashMap struct. I
feel like I have a slightly better understanding of it. It was very foreign feeling to pull
a value out of a HashMap in order to  mutate it's reference as the way to change a value at a key.

I'm used to `dict` in python where this is so intuitive.

```python
d = {'foo': 'bar'}
d['foo'] = 'baz'
```
