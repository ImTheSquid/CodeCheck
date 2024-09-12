def reverse(number):
    sisa = 0
    while number != 0:
        sisa = number % 10
        print(sisa, end='')
        number = number // 10
    print()

if __name__ == '__main__':
    angka = int(input("Enter an integer: "))
    reverse(angka)