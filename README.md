# two_pounds
This project was mostly for experimenting with Rust, and trying to solve the challenge listed at https://projecteuler.net/problem=31. This was my first Project Euler, and I think I kind of missed the point. I wanted to use Rust because I was interested in learning the language, but I took an object oriented approach which in retrospect was most likely a mistake, especially in a largely functional language like Rust. This is a cautionary tale.

My answer was correct for coins 1, 2, 5, 10, and 20, but broke on 50, 100, and 200.
My final answer for the 200 coin was 73478, which is close to the correct answer 73682 but no cigar. Also, doing it this way with the structs and hashset was quite inefficient. It took about 30 seconds to calculate that value on my arch VM. However, it does have the benefit of being able to print those combinations if you so desire - although this only makes sense for the lower coins.
