import random

a = [random.randint(1, 100) for _ in range(10)]
print("Read a number: ", a)
for i in range(len(a)-1, -1, -1):
  print(a[i])
