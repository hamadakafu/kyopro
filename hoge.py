s = input()
t = input()

i = zip(list(s), list(t))

while True:
    ss, tt = next(i, (-1, -1))
    if ss == -1 and tt == -1:
        break
    if ss != tt:
        sss, ttt = next(i)
        if ss == ttt and sss == tt:
            print('Yes')
            exit()
        else:
            print('No')
            exit()

print('Yes')
