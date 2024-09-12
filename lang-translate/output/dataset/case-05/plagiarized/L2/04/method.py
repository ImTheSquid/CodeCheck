def beautyReverse(num):
    while num != 0:
        r = num % 10
        print(r, end="")
        num = num // 10
    print()

if __name__ == "__main__":
    n = int(input("Enter an integer: "))
    beautyReverse(n)