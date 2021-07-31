x = list(map(int, list(input())))

if x[0] == x[1] == x[2] == x[3]:
    print("Weak")
    exit()

if (x[0] + 1) % 10 == x[1] and (x[1] + 1) % 10 == x[2] and (x[2] + 1) % 10 == x[3]:
    print("Weak")
    exit()

print("Strong")
