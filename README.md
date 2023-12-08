# advent-of-code-2023

I'm attempting Advent of Code 2023 using Rust. I'm pretty new to Rust so this is a really good opportunity to learn and have some fun!

I will be unit-testing all functions and prioritizing functionality over speed - if I get the right answer without having to wait half an hour I'll call that a success!

## Problem 1

### Part 1
This was a fairly simple problem but allowed me to understand the basics of unit-testing in Rust, as well as simple loops, looping over strings, casting strings to integers, and variable assignment for mutable and non-mutable variables. 

I don't fully understand the borrow checker yet, but that will come with time & research. I also didn't fully understand the .expect("" )

Solved 2023/12/09.

### Part 2
This was slightly harder and required a bit more thought. I tried to replace the word numbers with digits and then fall back to the original approach, however this didn't work for situations like "oneight" where it would convert it to "1ight" incorrectly. 

Therefore I had to change approach to looking for the index where particular substrings started, and this meant I had to implement different logic for the first and last occurances. Specifically this taught me about rfind vs find.

I also learned about match statements when trying to refine my solution.

This still isn't exactly as I'd like - in python I would have taken out the number_map as a global constant variable, but that led to errors I could not resolve. 

Solved 2023/12/09