def reverse(number):
    tempDigit = 0

    while number > 0:
        tempDigit = number % 10
        print(tempDigit, end="")
        number = number // 10

number = int(input("Enter an integer: "))
reverse(number)
