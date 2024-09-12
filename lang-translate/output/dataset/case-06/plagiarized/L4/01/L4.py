def proses():
    angka = []
    for a in range(10):
        angka.append(int(input("Read a number: ")))
    for b in range(9, -1, -1):
        print(angka[b])


if __name__ == "__main__":
    proses()