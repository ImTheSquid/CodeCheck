import numpy as np

def sumMajorDiagonal(m):
    hasil = 0
    for i in range(4):
        for j in range(4):
            m[i][j] = float(input())
            if i == j:
                hasil += m[i][j]
    return hasil

m = np.zeros((4, 4))
print("Enter a 4-by-4 matrix row by row:")
print("Sum of the elements in the major diagonal is " + str(sumMajorDiagonal(m)))