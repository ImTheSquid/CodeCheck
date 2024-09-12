def reverse(nmb):
    while nmb != 0:
        get = nmb % 10
        print(get, end="")
        nmb = nmb // 10
    print()

#enter the number
nmb = int(input("Enter an integer: "))
#reverse number
reverse(nmb)