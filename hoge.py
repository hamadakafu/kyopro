import math
p = float(input())

a = lambda p: - 3 / 2 * math.log2( 3 / (2 * p * math.log(2)))
f = lambda x, p: x + p / pow(2, x / 1.5)

print(f(max(a(p), 0), p))
