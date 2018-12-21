#!/usr/bin/env python3
import argparse
import os
import string
import tempfile
import textwrap
import shlex
import sys


def preamble():
    return textwrap.dedent('''\
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                int a = 1;
                int b = 0;
                int c = 0;
                int d = 0;
                int e = 0;
                int f = 0;\n''')


def footer():
    return textwrap.dedent('''\
                program_end:
                printf("[%d, %d, %d, %d, %d, %d]", a, b, c, d, e, f);
            }\n''')


def reg_name(num):
    return string.ascii_lowercase[num]


def exhaustive_jump(reg, base, lines):
    code = '{\n\tswitch (%s + %s) {\n' % (reg_name(reg), base)
    for i in range(lines):
        code += '\t\tcase %d: goto line_%d;\n' % (i, i)

    code += textwrap.dedent('''\
                    default: abort();
                }
            }''')

    return code


def get_var(ir, var, ip, i):
    if ir == 'i':
        return var
    elif var == ip:
        return i
    else:
        return reg_name(var)


def compile(lines):
    ip = int(next(lines).strip().split()[-1])

    lines = [line for line in lines]

    code = []

    cond = None

    for i, line in enumerate(lines):
        line = line.strip()
        parts = line.split()
        op = parts[0]
        var = [int(x) for x in parts[1:]]

        code_line = 'line_%d:\n' % i
        if cond is not None:
            code_line += '\t'
        if var[2] == ip:
            target = None
            target_reg = None
            if op == 'seti':
                target = var[0]
            elif op == 'addi':
                if var[0] == ip:
                    target = i + var[1]
                else:
                    sys.exit('Illegal jump at \'%s\'' % line)
            elif op == 'addr':
                if var[0] == ip and cond == var[1] \
                        or var[1] == ip and cond == var[0]:
                    target = i + 1
                else:
                    if var[0] == ip:
                        target_reg = var[1]
                    elif var[1] == ip:
                        target_reg = var[0]
                    else:
                        sys.exit('Illegal jump at \'%s\'' % line)
            elif op == 'mulr':
                if var[0] == ip and var[1] == ip:
                    target = i * i
                else:
                    sys.exit('Illegal jump at \'%s\'' % line)
            else:
                sys.exit('Illegal jump at \'%s\'' % line)

            cond = None
            if target is not None:
                if target + 1 < len(lines):
                    code_line += 'goto line_%d;' % (target + 1)
                else:
                    code_line += 'goto program_end;'
            elif target_reg is not None:
                code_line += exhaustive_jump(target_reg, i + 1, len(lines))
            else:
                sys.exit('Invalid compiler state')
        else:
            cond = None
            if op[:2] in ['gt', 'eq']:
                cond = var[2]
                if op[:2] == 'gt':
                    comp = '>'
                else:
                    comp = '=='

                var1 = get_var(op[2], var[0], ip, i)
                var2 = get_var(op[3], var[1], ip, i)
                code_line += 'if (%s %s %s)' % (var1, comp, var2)
            elif op[:3] == 'set':
                var1 = get_var(op[3], var[0], ip, i)
                target = reg_name(var[2])
                code_line += '%s = %s;' % (target, var1)
            else:
                var1 = get_var('r', var[0], ip, i)
                var2 = get_var(op[3], var[1], ip, i)
                target = reg_name(var[2])

                ops = {
                        'mul': '*',
                        'add': '+',
                        'bor': '|',
                        'ban': '&',
                        }

                op = ops[op[:3]]
                code_line += '%s = %s %s %s;' % (target, var1, op, var2)

        code.append(code_line)

    return preamble() + '\n'.join(code) + footer()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file')
    parser.add_argument('-t', '--transpile-only', action='store_true')
    parser.add_argument('-o')

    args = parser.parse_args()
    with open(args.file, 'rt') as f:
        code = compile(f)

    if args.transpile_only:
        print(code)
        return

    with tempfile.NamedTemporaryFile('wt', delete=True, suffix='.c') as f:
        f.write(code)
        f.flush()
        command = 'gcc -Wall -Wextra -O3'
        if args.o:
            command += ' -o ' + shlex.quote(args.o)

        command += ' ' + shlex.quote(f.name)
        os.system(command)


if __name__ == '__main__':
    main()
