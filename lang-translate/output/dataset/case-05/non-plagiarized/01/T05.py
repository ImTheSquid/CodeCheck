def reverse(number):
    while number > 0:
        print(number % 10, end="")
        number //= 10
    print()

number = int(input("Enter an integer: "))
reverse(number)