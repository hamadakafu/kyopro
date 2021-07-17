n = int(input())
s = input()

cnt = 0
for c in s:
    if c == '1':
        if cnt % 2 == 0:
            print('Takahashi')
            exit()
        else:
            print('Aoki')
            exit()
    cnt += 1
