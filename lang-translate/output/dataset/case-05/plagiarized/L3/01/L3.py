def balik(nomor):
    sisa = 0
    while nomor != 0:
        sisa = nomor % 10
        print(sisa, end='')
        nomor = nomor // 10
    print()

nomor = int(input("Enter an integer: "))
balik(nomor)