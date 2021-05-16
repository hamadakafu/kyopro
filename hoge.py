n = int(input())
yamas = []
for i in range(0, n):
    s, t = input().split(' ')
    yamas.append((s, int(t)))

yamas.sort(key= lambda x: x[1], reverse=True)
print(yamas[1][0])
