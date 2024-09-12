def beautyReverse(num):
    while num != 0:
        r = num % 10
        print(r, end="")
        num = num // 10
    print()

def reverse(n):
    beautyReverse(n)

n = int(input("Enter an integer: "))
reverse(n)