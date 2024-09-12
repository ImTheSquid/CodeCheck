import sys

angka = []

for i in range(10):
    angka.append(int(input("Read a number: ")))

for i in range(9, -1, -1):
    print(angka[i])