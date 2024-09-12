def sumMajorDiagonal(m):
  temp = 0
  for i in range(4):
    for j in range(4):
      if i == j:
        temp += m[i][j]
  return temp

m = [
  [1, 2, 3, 4],
  [5, 6, 7, 8],
  [9, 10, 11, 12],
  [13, 14, 15, 16]
]

print(f"Sum of the elements in the major diagonal is {sumMajorDiagonal(m)}")