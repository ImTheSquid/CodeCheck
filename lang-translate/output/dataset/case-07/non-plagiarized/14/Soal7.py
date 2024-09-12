def sumMajorDiagonal(m):
    hasil = 0
    for i in range(4):
        for j in range(4):
            if i == j:
                hasil += m[i][j]
    return hasil

m = [[0 for _ in range(4)] for _ in range(4)]
for i in range(4):
    for j in range(4):
        m[i][j] = float(input())

print("Sum of the elements in the major diagonal is " + str(sumMajorDiagonal(m)))