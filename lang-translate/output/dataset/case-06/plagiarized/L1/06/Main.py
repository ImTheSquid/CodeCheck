# This is a Python equivalent of the provided Java code

# Create a list to store the numbers
num = []

# Input 10 numbers from the user
for i in range(10):
    # Simulate user input, replace with input() to receive actual user input
    num.append(int(input(f"Read a number: ")))

# Print the numbers in reverse order
for i in range(9, -1, -1):
    print(num[i])
