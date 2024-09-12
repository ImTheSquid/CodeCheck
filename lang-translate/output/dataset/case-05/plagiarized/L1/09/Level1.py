def reverse(number):
    while number != 0:
        remainder = number % 10
        print(remainder, end='')
        number = number // 10
    print()

number = int(input("Enter an integer: "))
reverse(number)