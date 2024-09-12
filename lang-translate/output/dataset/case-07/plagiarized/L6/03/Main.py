import numpy as np

# Input matrix
matrix = np.array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])

# Calculate sum of diagonal elements
sum = np.trace(matrix)

# Print the result
print("Sum of the elements in the major diagonal is", sum)