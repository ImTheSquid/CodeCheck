import random

arr = [random.randint(0, 100) for _ in range(10)]
print("Read a number: ", end="")
print(arr)

for i in range(9, -1, -1):
    print(arr[i])