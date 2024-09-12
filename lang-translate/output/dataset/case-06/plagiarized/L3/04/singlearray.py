n = [0] * 10
for i in range(10):
  n[i] = int(input(f"Read a number: "))
for a in range(9, -1, -1):
  print(n[a])