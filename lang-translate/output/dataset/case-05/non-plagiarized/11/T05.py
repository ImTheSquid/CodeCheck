def reverse(number):
    """Reverses the digits of an integer."""
    temp = number
    count = 0
    while number != 0:
        number //= 10
        count += 1
    number = temp
    sum = 0
    for i in range(count, 0, -1):
        power_of_ten = 1
        last_digit = number % 10
        for j in range(1, i):
            power_of_ten *= 10
        sum += last_digit * power_of_ten
        number //= 10
    print(sum)

# Example usage:
number = 12345
reverse(number)