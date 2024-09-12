def HitungDiagonal(matriks):
  total = 0
  for i in range(len(matriks)):
    total += matriks[i][i]
  return total

# Get input from user
matrix = []
for _ in range(4):
  row = [float(x) for x in input().split()]
  matrix.append(row)

print(f"Sum of the elements in the major diagonal is {HitungDiagonal(matrix)}")