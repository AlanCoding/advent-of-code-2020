### Cheating in Advent of Code

I forked Caleb's repo so that I can practice rust by putting in my own
input to his programs.

I'm starting fresh on a Linux machine so I installed from scratch.

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

```
$ rustc --version
rustc 1.48.0 (7eac88abb 2020-11-16)
```

I added a Makefile to document how to compile and call the programs.
Generally, I'm adding an `input.txt` file to each day and running with that.

```
$ time make 1
rustc day1/src/main.rs -o day1/main
./day1/main day1/input.txt
part 1: 545379
part 2: 257778836

real	0m0.464s
user	0m0.497s
sys	0m0.050s
```

(half of the time is spent in compiling)
