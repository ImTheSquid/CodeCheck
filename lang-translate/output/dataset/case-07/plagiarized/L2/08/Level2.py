def hitungDiagonal(matrix):
  jumlah = 0
  for i in range(len(matrix)):
    jumlah += matrix[i][i]
  return jumlah

matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]
print(f"Sum of the elements in the major diagonal is {hitungDiagonal(matrix)}")