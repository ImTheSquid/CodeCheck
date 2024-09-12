import numpy as np

# Input the matrix
matrix = np.zeros((4, 4))
print("Enter a 4 by 4 matrix row by row: ")
for i in range(4):
    for j in range(4):
        matrix[i][j] = float(input())

# Calculate the sum of the diagonal elements
sum = 0
for i in range(4):
    sum += matrix[i][i]

# Print the result
print(f"Sum of the elements in the major diagonal is {sum}")