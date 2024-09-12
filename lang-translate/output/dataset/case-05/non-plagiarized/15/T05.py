import math

def reverse(number):
    a = 0
    i = int(math.log10(number))
    while number != 0:
        b = number % 10
        a += b * math.pow(10, i)
        i -= 1
        number //= 10
    print(a)

inp = 12345
reverse(inp)