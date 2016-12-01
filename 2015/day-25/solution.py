from __future__ import print_function

def findCode(n):
    modBase = 33554393
    base = 252533
    start = 20151125

    code = start * pow(base, n - 1, modBase)
    return code % modBase

row = 2947
col = 3029

triangleNo = row + col - 1
subTriangleSurface = ((triangleNo - 1) * triangleNo) // 2
n = subTriangleSurface + col

print(findCode(n))
