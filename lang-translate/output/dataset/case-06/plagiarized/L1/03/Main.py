def main():
    num = [0] * 10
    for i in range(10):
        num[i] = int(input("Read a number: "))
    for i in range(9, -1, -1):
        print(num[i])

if __name__ == "__main__":
    main()