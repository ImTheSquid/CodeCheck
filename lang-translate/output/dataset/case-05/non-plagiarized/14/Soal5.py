def reverse(number):
    a = str(number)
    for i in range(len(a) - 1, -1, -1):
        print(a[i], end="")
    print("")

number = int(input("Enter an integer: "))
reverse(number)
