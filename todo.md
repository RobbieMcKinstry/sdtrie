# TODO

Improve memory usage with the Wikipedia optimization

Add a DOT output so the trie can be visualized

Add a fuzzer

Improve lookup performance from O(n) to O(lg n)

Extend this type to allow other values to be stored on the leaves beyond IDs.

Benchmark and graph both memory usage and runtime


- [ ] Write a "timer" crate that contains
a channel one-shot, and a start time. When
it drops, get the end time and write the duration
to the channel.

- [ ] Write a package which takes a Fn which 
injects a timer, and writes to an output file
the return value and the time duration.




# DONE


