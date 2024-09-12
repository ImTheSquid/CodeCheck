def sumMajorDiagonal(m):
  hasil = 0
  for i in range(4):
    hasil += m[i][i]
  return hasil

m = []
for i in range(4):
  row = []
  for j in range(4):
    row.append(float(input()))
  m.append(row)

print(f"Sum of the elements in the major diagonal is {sumMajorDiagonal(m)}")
