#input data
angka = int(input("Enter an integer: "))

#proses membalikkan angka
while (angka != 0):
    #tampilkan angka yang diambil
    print(angka % 10, end='')
    #membuang angka terakhir
    angka = angka // 10
print()