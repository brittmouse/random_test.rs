# random_test.rs
A very small project for practicing Rust's std::io, random number generation, and vectors.

## Why I made this
I have written this problem up in Python and JS previously, and both times it was pretty simple. I had heard before that Python's pseudo random number generation did not produce numbers that had an even distribution of values that had been modulus-ed by 107; I wanted to test this for myself. After I had written one bad and one half-decent attempt, I wanted to reproduce the concept in other languages.
(N.B., I did not find the same scuffed dispersion of values in the Python test, perhaps my parameters were different.)

## How it works
The code is pretty simple when reading it. The program takes two u32 inputs, which determine the modulo and how many times to generate a random number from 1 to 1 million inclusive. The random number is generated and put through the modulus operation. The first version of the program used a `Vec<u32>` to store the values, but that solution felt awkward after solving this problem in Java.
The new iteration uses `HashMap<u32, u32>` to store the key (the modulo result) and the value (how many times that result occurred). This is similar to the way maps / dicts work in Java, JS, and Python, although the syntax is rather different. The map solution came up after I wrote the program in Java, where messing with the ArrayList I was appending all my results to proved too much of a hassle. I also added a `printMap` function that just prints the key-value pair in a more readable way.

Testing the program reveals the same sort of answers as the Python and JS versions. There is never a completely flat distribution curve, but looping enough times causes dispersion to drop substantially as expected.
The program will only take numeric inputs greater than 0 and will not perform the main loop otherwise.

I still believe this is pretty beginner Rust, but this is a cleaner implementation of the program than the vector used previously. It is also faster, as this version can handle the large variable inputs that the previous version couldn't without killing your threads (this wasn't tested scientifically, but printing the items in the map was much faster than iterating through the vec and printing with large modulus arguments). There may be other ways to clean it up or make it faster or safer but I am satisfied with this for the time being. This has become one of my favorite things to do when looking at a new language, and I plan on doing it with every language I try, especially C and Go (maybe Kotlin or Clojure if I ever decide to delve into Lisp-craft).
