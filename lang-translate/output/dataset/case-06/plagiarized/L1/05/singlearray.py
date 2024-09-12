# This is a Python version of the Java code provided.

numbers = []  # Initialize an empty list to store the numbers
for i in range(10):  # Loop 10 times to get 10 numbers
    number = int(input(f"Read a number: "))  # Get input from the user and convert it to an integer
    numbers.append(number)  # Add the input number to the list

for i in range(9, -1, -1):  # Loop through the list in reverse order
    print(numbers[i])  # Print each number from the list in reverse order