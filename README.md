# random_test.rs
A very small project for practicing Rust's std::io, random number generation, and vectors.

## Why I made this
I have written this problem up in Python and JS previously, and both times it was pretty simple. I had heard before that Python's pseudo random number generation did not produce numbers that had an even distribution of values that had been modulus-ed by 107; I wanted to test this for myself. After I had written one bad and one half-decent attempt, I wanted to reproduce the concept in other languages.
(N.B., I did not find the same scuffed dispersion of values in the Python test, perhaps my parameters were different.)

## How it works
The code is pretty simple when reading it. The program takes two u32 inputs, which determine the modulo and how many times to generate a random number from 1 to 1 million inclusive. The random number is generated, put through the modulus operation, and pushed to a u32 vector. Once the loop is done, a new loop starts by going from 0 to modulo - 1, counts all values in the vector that equal the loop variable, and prints the value and count.
Testing the program reveals the same sort of answers as the Python and JS versions. There is never a completely flat distribution curve, but looping enough times causes dispersion to drop substantially as expected.
The program will only take numeric inputs greater than 0 and will not perform the main loop otherwise.

This is very beginner Rust but I enjoyed writing this. There may be better ways to optimize this that I am unaware of so I imagine I'm going to look at this in two months and wince a bit. If I do end up wincing then I know I will have gotten better. Enjoy!
