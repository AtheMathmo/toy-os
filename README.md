# toy-os

This project is a playground for building an OS. I originally began this project whilst following alongside some [lecture notes](https://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf) for writing a simple operating system from scratch.

Once I had a simple C kernel working I decided to switch to [Rust](https://www.rust-lang.org/)! This was a fairly painful transition initially. Adapting my boot loader proved to be very error prone. Eventually I switched to GRUB and began following [Philipp Oppermann's awesome blog](http://os.phil-opp.com/). Which is crazy good.

I'm still trying to implement most things myself - or atleast ensure I have a good understanding of anything I adopt from the blog.

## Goals

My immediate goals are to achieve the following (roughly in order):

1. Implement a heap.
2. Better exception handling (currently partially completed).
3. Some basic drivers.
4. File system.

If it is remotely possible I'll be looking at throwing machine learning into the mix. If anyone has any ideas on how that could be achieved I'd love to hear them!

## TODOS:

Things on the immediate checklist.

- Write code to parse the grub multiboot headers.
- Use the headers to set the flags on the kernel remap function.
- Write a heap allocator!
