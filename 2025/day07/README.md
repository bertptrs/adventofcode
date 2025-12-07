# Day 07: Haskell

This one took me the longest, mostly because I really do not know Haskell. Nevertheless, I find the
resulting solution quite elegant. The `Makefile` I made uses dynamic linkage as that's how Haskell
works on Arch Linux. If you want to do static linking, `ghc solve.hs` should work.

```console
$ make
ghc -dynamic -O -g -o solve solve.hs
[1 of 2] Compiling Main             ( solve.hs, solve.o )
[2 of 2] Linking solve
$ ./solve sample.txt
21
40
```
