def beautyReverse(num):
    while num != 0:
        r = num % 10
        print(r, end="")
        num //= 10
    print()

def reverse(n):
    beautyReverse(n)

if __name__ == "__main__":
    n = int(input("Enter an integer: "))
    reverse(n)