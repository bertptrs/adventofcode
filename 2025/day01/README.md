# Day 01: Nix

Nix is a functional language made for the Nix package manager. As 

To run the solution program, start the `nix` repl with the solution program and call the `solve`
function with the path to the input file.

```console
$ nix repl --option max-call-depth 10000 --file solve.nix
nix-repl> solve ./sample.txt
```

Some observations:

- The `max-call-depth` needs to be bumped to at least 10k for the main input files, otherwise you
  hit the limit in the recursion.
- The standard library is lacking several basics but most of those you can build yourself
