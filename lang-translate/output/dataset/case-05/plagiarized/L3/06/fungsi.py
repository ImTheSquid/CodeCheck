def terbalik(angka):
    while angka != 0:
        sisa = angka % 10
        print(sisa, end="")
        angka = angka // 10
    print()

if __name__ == "__main__":
    input = int(input("Enter an integer: "))
    terbalik(input)