def showArray(angka):
    # Menampilkan array
    i = 0
    while i <= 9:
        print(angka[i])
        i += 1

if __name__ == "__main__":
    angka = [0] * 10
    for i in range(10):
        # Input angka pada bagian ini
        angka[i] = int(input("Read a number: "))
    showArray(angka)