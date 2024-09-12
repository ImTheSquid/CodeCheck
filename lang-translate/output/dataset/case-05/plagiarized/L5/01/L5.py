def main():
    nomor = int(input("Enter an integer: "))
    sisa = 0
    while nomor != 0:
        sisa = nomor % 10
        print(sisa, end="")
        nomor = nomor // 10
    print()

if __name__ == "__main__":
    main()