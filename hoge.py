n = int(input())

aa = list(map(int, input().split()))

ans = 0
for i in range(0, n):
    if aa[i] > 10:
        ans += aa[i] - 10

print(ans)
