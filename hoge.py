n = int(input())

tmp = 0
count = 1
while tmp < n:
    tmp += count
    count += 1

print(count - 1)
