SOLUTIONS=$(addsuffix /solution, $(wildcard day-*))
TESTS=$(sort $(patsubst day-%/solution, test-%, $(SOLUTIONS)))

.PHONY: all clean test

all: $(SOLUTIONS)

clean:
	$(RM) $(SOLUTIONS)

day-%/solution: day-%/solution.rs
	rustc -o $@ $^

# Utility target to run all the programs
test: $(TESTS)

test-%: day-%/solution
	$^ $(dir $<)input.txt
