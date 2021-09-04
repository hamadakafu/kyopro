s1 = input()
s2 = input()
s3 = input()

ss = set()
ss.add('ABC')
ss.add('ARC')
ss.add('AGC')
ss.add('AHC')
ss.remove(s1)
ss.remove(s2)
ss.remove(s3)
for s in ss:
    print(s)
