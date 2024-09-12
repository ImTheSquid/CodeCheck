def reverse(angka):
    # Di sini mengulang hingga number habis
    while (angka != 0):
        remainder = angka % 10
        print(remainder, end='')
        angka = angka // 10
    print()

angka = int(input("Enter an integer: "))
# Memanggil fungsi reverse untuk membalik nilai yang dihasilkan
reverse(angka)