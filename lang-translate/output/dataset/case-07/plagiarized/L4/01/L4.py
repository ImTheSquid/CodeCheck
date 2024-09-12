def jumlahDiagonal(m):
  sum = 0
  for i in range(len(m)):
    sum += m[i][i]
  return sum

def print(m):
  print(f"Sum of the elements in the major diagonal is {jumlahDiagonal(m)}")

# Example matrix for testing
m = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]
print(m)