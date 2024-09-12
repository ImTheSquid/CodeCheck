import numpy as np

# Input the matrix
matrix = np.array([
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12],
    [13, 14, 15, 16]
])

# Calculate the sum of the major diagonal
sum = np.trace(matrix)

# Print the result
print(f"Sum of the elements in the major diagonal is {sum}")