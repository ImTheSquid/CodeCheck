# Program to read 10 numbers and display them in reverse order

# Declare an array of size 10
arrNum = [0] * 10

# Read 10 numbers from the user
for i in range(10):
    arrNum[i] = int(input(f"Read a number: "))

# Display the numbers in reverse order
for i in range(9, -1, -1):
    print(arrNum[i])