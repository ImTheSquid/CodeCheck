def reverse(number):
    if number > 0:
        print(number % 10, end='')
        reverse(number // 10)

number = int(input("Enter an integer: "))
reverse(number)
print("")