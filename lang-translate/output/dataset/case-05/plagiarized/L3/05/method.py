def beautyReverse(num):
    while num != 0:
        r = num % 10
        print(r, end='')
        num = num // 10
    print()

# prog utama
n = int(input("Enter an integer: "))
# pamggil method
beautyReverse(n)