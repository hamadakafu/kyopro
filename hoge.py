a, b, ab, x, y = map(int, input().split(' '))

if (a + b) / 2 > ab:
# abを買ったほうがお得
  abmai = min(x, y) * 2
  more = max(x, y) * ab * 2
  if x > y:
    print(int(min(abmai * ab + a * (x - (abmai / 2)), more)))
  else:
    print(int(min(abmai * ab + b * (y - (abmai / 2)), more)))

  exit()

else:
  print(a * x + b * y)
