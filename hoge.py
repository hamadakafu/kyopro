n, p = map(int, input().split())
aa = list(map(int, input().split()))

ans = 0
for a in aa:
    if a < p:
        ans += 1

print(ans)
