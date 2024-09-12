def HitungDiagonal(matriks):
  total = 0
  for i in range(len(matriks)):
    total += matriks[i][i]
  return total


if __name__ == "__main__":
  matrix = []
  print("Enter a 4 by 4 matrix row by row: ")
  for _ in range(4):
    row = [float(x) for x in input().split()]
    matrix.append(row)

  hasil = HitungDiagonal(matrix)
  print("Sum of the elements in the major  diagonal is  ", hasil)
