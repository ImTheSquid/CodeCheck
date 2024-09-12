def reverseNumber(angka):
    while angka != 0:
        temp = angka % 10
        print(temp, end="")
        angka = angka // 10
    print()

angka = int(input("Enter an integer: "))
reverseNumber(angka)
