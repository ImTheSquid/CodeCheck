def sumMajorDiagonal(m):
  sum = 0
  for i in range(4):
    sum += m[i][i]
  return sum

matrix = []
for i in range(4):
  row = []
  print(f'Enter row {i+1}:')
  for j in range(4):
    row.append(float(input()))
  matrix.append(row)

print(f"Sum of the elements in the major diagonal is {sumMajorDiagonal(matrix)}")