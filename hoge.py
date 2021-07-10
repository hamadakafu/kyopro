n, x = map(int, input().split())
aa = [a for a in map(int, input().split())]
for i in range(0, n):
    if i % 2 == 0:
        continue
    else:
        aa[i] -= 1

for a in aa:
    x -= a
    if x < 0:
        print('No')
        exit(0)

print('Yes')
