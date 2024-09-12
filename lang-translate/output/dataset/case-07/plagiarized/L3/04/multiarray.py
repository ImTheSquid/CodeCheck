import numpy as np

def sum(m):
    sum = 0
    for i in range(len(m)):
        sum += m[i][i]
    return sum

if __name__ == '__main__':
    ma = np.zeros((4, 4))
    print("Enter a 4 by 4 matrix row by row: ")
    for i in range(4):
        for j in range(4):
            ma[i][j] = float(input())
    print(f"Sum of the elements in the major diagonal is {sum(ma)}")