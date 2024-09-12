import random

angka = [random.randint(0, 100) for _ in range(10)] 

print("Read a number: ")
for i in range(10):
    print(angka[i])

for i in range(9, -1, -1):
    print(angka[i])