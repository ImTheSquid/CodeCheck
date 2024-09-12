def reverse(number):
    return number % 10

number = 12345
for _ in range(number):
    print(reverse(number), end="")
    number //= 10
print("")