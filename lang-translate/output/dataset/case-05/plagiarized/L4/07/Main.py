angka = int(input("Enter an integer: "))

while angka != 0:
    remainder = angka % 10
    print(remainder, end='')
    angka = angka // 10

print()