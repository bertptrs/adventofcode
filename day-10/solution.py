def lookandsay(line):
    p = None
    n = 0
    result = []
    for c in line:
        if n > 0 and p is not c:
            result.append(str(n))
            result.append(p)
            n = 0

        p = c
        n += 1

    result.append(str(n))
    result.append(p)

    return ''.join(result)


line = "1321131112"
for x in range(40):
    line = lookandsay(line)

print "40:", len(line)

for x in range(10):
    line = lookandsay(line)

print "50:", len(line)
