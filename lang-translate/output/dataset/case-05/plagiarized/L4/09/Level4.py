def main():
    number = int(input("Enter an integer: "))
    while number != 0:
        sisaBagi = number % 10
        print(sisaBagi, end='')
        number = number // 10
    print()

if __name__ == '__main__':
    main()