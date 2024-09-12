angka = int(input("Enter an integer: "))
for angka in range(angka, 0, -1):
    remainder = angka % 10
    print(remainder, end="")
print()
