def sumMajorDiagonal(m):
  sum = 0
  for i in range(4):
    for j in range(4):
      if i == j:
        sum += m[i][j]
  return sum

# Test with example matrix (replace with user input)
a = [[1, 2, 3, 4],
     [5, 6, 7, 8],
     [9, 10, 11, 12],
     [13, 14, 15, 16]]

print("Sum of the elements in the major diagonal is", sumMajorDiagonal(a))