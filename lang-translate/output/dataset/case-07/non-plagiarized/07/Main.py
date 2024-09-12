import numpy as np

def sumMajorDiagonal(array):
  total = 0
  for index in range(len(array)):
    total += array[index][index]
  return total

a = np.array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])

for i in range(4):
  for j in range(4):
    print(a[i][j], end=" ")
  print()

print(f"Sum of the elements in the major diagonal is {sumMajorDiagonal(a)}")