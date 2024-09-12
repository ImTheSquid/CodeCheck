import numpy as np

def sumMajorDiagonal(m):
  total = np.trace(m)
  print(f"Sum of the elements in the major diagonal is {total}")
  return total

arr = np.array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])
sumMajorDiagonal(arr)
