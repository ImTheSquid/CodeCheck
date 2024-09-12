def main():
    nomor = 0
    print("Enter an integer: ")
    nomor = int(input())
    sisa = 0
    while nomor != 0:
        sisa = nomor % 10
        print(sisa, end="")
        nomor = nomor // 10
    print()

if __name__ == "__main__":
    main()