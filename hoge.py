pp = list(map(int, input().split()))
ss = ""
for i in range(26):
    ss += chr(ord("a") - 1 + pp[i])
print(ss)
