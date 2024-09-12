def main():
    input = int(input("Enter an integer: "))
    while input != 0:
        sisa = input % 10
        print(sisa, end="")
        input = input // 10

if __name__ == "__main__":
    main()