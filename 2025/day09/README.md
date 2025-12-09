# Day 9: Typescript

Lost a lot of time on a swapped argument order. Oh well, such is life. The `package.json` exists to
instruct the interpreter on how to execute the file and doesn't otherwise include any meaningful
dependencies. Everything works with the standard library for Node.

`ts-node` is used for just-in-time Typescript compilation. You can also compile the file manually
first, then run it as JS. Also, Oracle, please release the name Javascript.

```console
$ ./solve.ts sample.txt
Part 1: 50
Part 2: 24
```
