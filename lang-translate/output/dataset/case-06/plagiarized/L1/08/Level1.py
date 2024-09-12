# This code reads 10 numbers from the user and prints them in reverse order.
numbers = []
for i in range(10):
    number = int(input(f"Read a number: "))
    numbers.append(number)

# Print the numbers in reverse order
for i in range(len(numbers) - 1, -1, -1):
    print(numbers[i])