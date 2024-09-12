def sumMajorDiagonal(m):
  result = 0
  for i in range(4):
    result += m[i][i]
  return result

matrix = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
print("Enter a 4-by-4 matrix row to row : ")
for i in range(4):
  for j in range(4):
    matrix[i][j] = float(input())

result = sumMajorDiagonal(matrix)
print("Sum of the element in the major diagonal is " + str(result))
