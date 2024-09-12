import numpy as np

def sumMajorDiagonal(matrix, n):
    sum = 0
    for i in range(n):
        sum += matrix[i][i]
    print("Sum of the elements in the major diagonal is " + str(sum))

n = 4
print("Enter a 4 by 4 matrix row by row: ")
matrix = np.zeros((n, n))
for i in range(n):
    row = input().split()
    for j in range(n):
        matrix[i][j] = float(row[j])

sumMajorDiagonal(matrix, n)