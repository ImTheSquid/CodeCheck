# This is the code that does the same thing as the java code
# The code is written in python 3
# This code takes 10 integer inputs and prints them in reverse order

numbers = []
for i in range(10):
    number = int(input("Read a number: "))
    numbers.append(number)

for i in range(10):
    print(numbers[9 - i])