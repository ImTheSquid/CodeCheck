import random

Array = []
for i in range(10):
  Array.append(random.randint(0, 100))

for i in range(9, -1, -1):
  print(Array[i])