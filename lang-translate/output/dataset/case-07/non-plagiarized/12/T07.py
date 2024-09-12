def sumMajorDiagonal(m):
  """Calculates the sum of the elements in the major diagonal of a 4x4 matrix.

  Args:
    m: A 4x4 matrix represented as a list of lists.

  Returns:
    The sum of the elements in the major diagonal.
  """
  tambah = m[0][0] + m[1][1] + m[2][2] + m[3][3]
  return tambah

# Example usage:
m = [[1, 2, 3, 4],
     [5, 6, 7, 8],
     [9, 10, 11, 12],
     [13, 14, 15, 16]]
print("Sum of the elements in the major diagonal is", sumMajorDiagonal(m))