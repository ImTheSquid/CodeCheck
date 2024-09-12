def main():
    nomor = 0
    sisa = 0
    nomor = int(input("Enter an integer: "))
    
    while (nomor!=0):    
        if (nomor!=0):
            sisa = nomor % 10
            print(sisa, end="")
            nomor = nomor // 10
    print()

if __name__ == "__main__":
    main()