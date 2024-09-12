def terbalik(angka):
    while angka > 0:
        sisa = angka % 10
        print(sisa, end='')
        angka //= 10

input = int(input("Enter an integer: "))
terbalik(input)
print()