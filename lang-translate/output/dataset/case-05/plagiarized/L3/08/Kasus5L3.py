def reverseNumber(angka):
    while angka != 0:
        temp = angka % 10
        angka = angka // 10
        print(temp, end="")
    print()

angka = int(input("Enter an integer: "))
reverseNumber(angka)
