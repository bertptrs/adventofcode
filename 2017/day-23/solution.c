#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_INSTR 1000

#define SETVAL

typedef struct {
	char op[4];
	char op_a[2];
	char op_b[10];
} instr_t;

int get_val(const char* identifier, const int* registers)
{
	if (isalpha(identifier[0])) {
		return registers[identifier[0] - 'a'];
	} else {
		return atoi(identifier);
	}
}

int run_program(const instr_t program[], const int max_instr, int registers[])
{
	int iptr = 0;
	int muls = 0;
	while (iptr < max_instr) {
		instr_t const* cur = &program[iptr];
		int op_b = get_val(cur->op_b, registers);

		int* res = &registers[cur->op_a[0] - 'a'];

		if (!strcmp(cur->op, "set")) {
			*res = op_b;
		} else if (!strcmp(cur->op, "sub")) {
			*res -= op_b;
		} else if (!strcmp(cur->op, "mul")) {
			*res *= op_b;
			++muls;
		} else if (!strcmp(cur->op, "jnz")) {
			if (get_val(cur->op_a, registers) != 0) {
				iptr += op_b;
				continue;
			}
		}
		iptr += 1;
	}

	return muls;
}

int is_prime(long long int num) {
	for (long long int i = 2; i * i <= num; ++i) {
		if (num % i == 0) {
			return 0;
		}
	}

	return 1;
}

int run_optimized()
{
	int b = 84 * 100 + 100000;
	int c = b + 17000;
	int h = 0;

	for (; b <= c; b += 17) {
		// Test if B is prime, result = f == 1;
		if (!is_prime(b)) {
			h += 1;
		}
	}

	return h;
}

int main()
{
	instr_t program[MAX_INSTR];
	int max_instr = 0;

	int debug_registers[8] = {0};

	while (scanf("%s %s %s", program[max_instr].op, program[max_instr].op_a, program[max_instr].op_b) != EOF) {
		++max_instr;
	}

	int muls = run_program(program, max_instr, debug_registers);
	printf("Muls executed: %d\n", muls);

	printf("Final value of 'h': %d\n", run_optimized());

	return 0;
}
