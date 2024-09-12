import numpy as np

def sumMajorDiagonal(m):
    sum = 0
    for i in range(len(m)):
        sum += m[i][i]
    return sum

#input the matrix
m = np.array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])

print("Sum of the elements in the major diagonal is", sumMajorDiagonal(m))