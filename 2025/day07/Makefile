.PHONY: all clean test

all: solve

test: solve
	./solve sample.txt

solve: solve.hs
	ghc -dynamic -O -o $@ $^

clean:
	$(RM) solve
	$(RM) *.o
	$(RM) *.hi
