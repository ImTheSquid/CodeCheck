def sumMajorDiagonal(m):
  jumdiagonal = 0
  for i in range(4):
    for j in range(4):
      if i == j:
        jumdiagonal += m[i][j]
  return jumdiagonal

numbers = []
print("Enter a 4-by-4 matrix row by row:")
for i in range(4):
  rows = input()
  rowssplit = rows.split()
  numbers.append([float(x) for x in rowssplit])

print("Sum of the elements in the major diagonal is", sumMajorDiagonal(numbers))