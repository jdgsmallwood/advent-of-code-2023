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

## Problem 2

### Part 1
This was a good one to get used to string manipulations like splitting in Rust. I skipped splitting out one function and unit testing it and there was a small error that I didn't find that was causing me to get the wrong answer multiple times. While I am learning it is a good idea to break everything out and check I am getting the output I desire.

Solved 2023/12/09

### Part 2
This was pretty easy - the problem just needed the maximum number of each type of cube in the game. Some rearrangement of the logic and a couple of new functions solved it.

I also learned a bunch about how to write this more idiomatically by using more functional-style calls, including use of fold.

Solved 2023/12/10