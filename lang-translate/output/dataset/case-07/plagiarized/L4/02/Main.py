import numpy as np

def inputData(mtx):
    for i in range(4):
        for j in range(4):
            mtx[i][j] = float(input())

def sumMajorDiagonal(mtx):
    sum = 0
    for i in range(len(mtx)):
        sum += mtx[i][i]
    return sum

mtx = np.zeros((4, 4))
print("Enter a 4 by 4 matrix row by row: ")
inputData(mtx)
print(f"Sum of the elements in the major diagonal is {sumMajorDiagonal(mtx)}")