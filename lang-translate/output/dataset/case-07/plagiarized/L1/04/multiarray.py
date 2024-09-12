import numpy as np

def sumMajorDiagonal(m):
  """
  This function calculates the sum of the elements in the major diagonal of a 2D array.

  Args:
    m: A 2D array.

  Returns:
    The sum of the elements in the major diagonal of the array.
  """
  sum = 0
  for i in range(len(m)):
    sum += m[i][i]
  return sum

# Example usage
m = np.array([[1, 2, 3, 4],
             [5, 6, 7, 8],
             [9, 10, 11, 12],
             [13, 14, 15, 16]])
print(f"Sum of the elements in the major diagonal is {sumMajorDiagonal(m)}")