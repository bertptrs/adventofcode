import fileinput

print("digraph day24 {")

for line in fileinput.input():
    parts = line.split(" ")
    if len(parts) != 5:
        continue

    first, op, second, _, result = parts
    print(f'{first}{second}{op} [label="{op}"];')
    print(f"{first} -> {first}{second}{op} -> {result};")
    print(f"{second} -> {first}{second}{op};")

print("}")
