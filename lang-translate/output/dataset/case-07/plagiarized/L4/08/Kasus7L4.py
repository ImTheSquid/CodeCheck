import numpy as np

#minta input
matrix = np.zeros((4, 4))
print("Enter a 4 by 4 matrix row by row: ")

#input the matrix
for i in range(4):
    for j in range(4):
        matrix[i][j] = float(input())

sum = 0
for i in range(4):
    sum += matrix[i][i]
print(f"Sum of the elements in the major diagonal is {sum}")