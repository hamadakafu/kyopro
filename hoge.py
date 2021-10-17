s = input()
mins = str(s[:])
maxs = str(s[:])

def rotate(l, n):
    return l[-n:] + l[:-n]

for _ in range(len(s)):
    s = rotate(s, 1)
    mins = min(mins, s)
    maxs = max(maxs, s)

print(mins, maxs)
