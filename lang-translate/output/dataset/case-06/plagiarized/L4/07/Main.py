def showArray(angka):
    # Menampilkan array
    for i in range(9, -1, -1):
        print(angka[i])

angka = []
for i in range(10):
    # Input angka pada bagian ini
    angka.append(int(input("Read a number: ")))
showArray(angka)