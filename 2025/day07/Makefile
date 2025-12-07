.PHONY: all clean test

all: solve

test: solve
	./solve sample.txt

solve: solve.hs
	ghc -dynamic -O -g -o $@ $^

clean:
	$(RM) solve
	$(RM) *.o
	$(RM) *.hi
