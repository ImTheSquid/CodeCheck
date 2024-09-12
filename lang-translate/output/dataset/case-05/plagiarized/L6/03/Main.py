number = int(input("Enter an integer: "))
for i in range(99999, 0, -1):
    remainder = number % 10
    print(remainder, end='')
    number = number // 10
print()