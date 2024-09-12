# This is a Python equivalent of the Java code.

numbers = []

for i in range(10):
    number = int(input("Read a number: "))
    numbers.append(number)

for i in range(9, -1, -1):
    print(numbers[i])