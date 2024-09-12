angka = 12345
while angka != 0:
    temp = angka % 10
    angka = angka // 10
    print(temp, end="")
print()