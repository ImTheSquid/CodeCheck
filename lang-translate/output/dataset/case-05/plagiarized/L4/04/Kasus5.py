def reverse(number):
    print(number % 10, end="")

number = int(input("Enter an integer: "))

while number > 0:
    reverse(number)
    number //= 10

print("")