import math
a, b, c, d = map(int, input().split())

if c * d - b <= 0:
    if a == 0:
        print(0)
    else:
        print(-1)
else:
    print(math.ceil((a / (c * d - b))))

