a = []
for i in range(10):
  number = int(input(f"Read a number: "))
  a.append(number)

for i in range(9, -1, -1):
  print(a[i])
