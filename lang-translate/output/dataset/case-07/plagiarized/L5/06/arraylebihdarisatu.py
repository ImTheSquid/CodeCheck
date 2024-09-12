def HitungDiagonal(matriks):
  total = 0
  i = 0
  while i < len(matriks):
    total += matriks[i][i]
    i += 1
  return total

if __name__ == "__main__":
  matrix = []
  print("Enter a 4 by 4 matrix row by row: ")
  for x in range(4):
    row = []
    for y in range(4):
      row.append(float(input()))
    matrix.append(row)
  hasil = HitungDiagonal(matrix)
  print("Sum of the elements in the major diagonal is ", hasil)