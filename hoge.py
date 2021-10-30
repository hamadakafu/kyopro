n = int(input())

f1, f2 = map(int, input().split())
a, b = map(int, input().split())
check = 0
if f1 == a or f2 == a:
    check = a
elif f1 == b or f2 == b:
    check = b
else:
    print('No')
    exit()


for _ in range(n - 3):
    a, b = map(int, input().split())
    if a == check or b == check:
        continue
    else:
        print('No')
        exit()

print('Yes')

