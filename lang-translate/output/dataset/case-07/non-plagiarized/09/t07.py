import numpy as np

def sumMajorDiagonal(m):
  """
  Calculates the sum of the elements in the major diagonal of a 4x4 matrix.

  Args:
    m: A 4x4 matrix represented as a nested list.

  Returns:
    The sum of the elements in the major diagonal.
  """
  hasil = 0
  for i in range(4):
    hasil += m[i][i]
  return hasil

if __name__ == "__main__":
  matriks = np.zeros((4, 4))
  print("Enter a 4-by-4 matrix row by row: ")
  for i in range(4):
    for j in range(4):
      matriks[i][j] = float(input())
  print(f"Sum of the elements in the major diagonal is {sumMajorDiagonal(matriks)}")