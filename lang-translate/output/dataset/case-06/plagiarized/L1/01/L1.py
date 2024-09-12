def main():
    num = []
    for i in range(10):
        num.append(int(input(f"Read a number: ")))

    for i in range(9, -1, -1):
        print(num[i])

if __name__ == "__main__":
    main()