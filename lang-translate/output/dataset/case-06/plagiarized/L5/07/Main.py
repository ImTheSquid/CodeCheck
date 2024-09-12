def showArray(angka):
    i = 9
    while i >= 0:
        print(angka[i])
        i -= 1

angka = []
for i in range(10):
    angka.append(int(input("Read a number: ")))

showArray(angka)