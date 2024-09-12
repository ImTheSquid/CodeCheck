def inputData(mtx):
    i = 0
    j = 0
    while i < 4:
        while j < 4:
            mtx[i][j] = float(input())
            j += 1
        j = 0
        i += 1


def sumMajorDiagonal(mtx):
    sum = 0
    for i in range(0, len(mtx), 2):
        sum += mtx[i][i] + mtx[i + 1][i + 1]
    return sum


if __name__ == "__main__":
    mtx = [[0 for _ in range(4)] for _ in range(4)]
    print("Enter a 4 by 4 matrix row by row: ")
    inputData(mtx)
    print("Sum of the elements in the major diagonal is " + str(sumMajorDiagonal(mtx)))
