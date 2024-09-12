def sumMajorDiagonal(m):
    sum = 0
    for i in range(len(m)):
        sum += m[i][i]
    return sum

arr = []
print("Enter a 4 by 4 matrix row by row: ")
for i in range(4):
    row = []
    for j in range(4):
        row.append(float(input()))
    arr.append(row)

print("Sum of the elements in the major diagonal is " + str(sumMajorDiagonal(arr)))
