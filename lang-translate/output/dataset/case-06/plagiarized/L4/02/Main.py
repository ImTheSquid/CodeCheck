def printNumber(number):
    for i in range(9, -1, -1):
        print(number[i])

def main():
    number = [0] * 10

    for i in range(10):
        number[i] = int(input("Read a number: "))

    printNumber(number)

if __name__ == "__main__":
    main()