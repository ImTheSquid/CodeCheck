import numpy as np

def HitungDiagonal():
    matrix = np.zeros((4, 4))
    print("Enter a 4 by 4 matrix row by row: ")
    for i in range(4):
        for j in range(4):
            matrix[i][j] = float(input())
    hasil = np.trace(matrix)
    return hasil

if __name__ == "__main__":
    print(f"Sum of the elements in the major diagonal is {HitungDiagonal()}")