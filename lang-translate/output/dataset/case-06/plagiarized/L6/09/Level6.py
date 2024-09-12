import random

def tampil(angka):
    for x in range(10, 0, -1):
        print(angka[x])

def main():
    arrAngka = [random.randint(0, 100) for _ in range(10)]

    for x in range(len(arrAngka)):
        print(f"Read a number: {arrAngka[x]}")

    tampil(arrAngka)

if __name__ == "__main__":
    main()