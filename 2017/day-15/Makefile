all: solution
	./$<

solution: solution.hs
	ghc -dynamic $< -o $@
