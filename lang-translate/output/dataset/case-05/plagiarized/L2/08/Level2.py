def balik(angka):
    while angka != 0:
        sisa = angka % 10
        print(sisa, end='')
        angka = angka // 10
    print()

angka = int(input("Enter an integer: "))
balik(angka)