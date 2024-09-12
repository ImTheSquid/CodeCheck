number = int(input("Enter an integer: "))
for i in range(0, number):
    remainder = number % 10
    print(remainder, end='')
    number = number // 10

print()