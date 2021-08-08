from operator import xor

n = int(input())
aa = list(enumerate(map(int, input().split())))

aa.sort(key=lambda x: x[1], reverse=True)
print(aa[1][0] + 1)
