def input():
    nmb = int(input("Enter an integer: "))
    return nmb

def reverse(nmb):
    get = 0
    while nmb != 0:
        get = nmb % 10
        print(get, end="")
        nmb = nmb // 10
    print()

nmb = input()
reverse(nmb)