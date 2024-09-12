import numpy as np

def sum(m):
  """
  This function calculates the sum of the elements in the major diagonal of a 2D array.
  """
  sum = 0
  for i in range(len(m)):
    sum += m[i][i]
  return sum

def main():
  """
  This function takes a 4x4 matrix from user input and calculates the sum of its diagonal elements.
  """
  print("Enter a 4 by 4 matrix row by row: ")
  ma = []
  for i in range(4):
    row = list(map(float, input().split()))
    ma.append(row)
  ma = np.array(ma)
  sum_diagonal = sum(ma)
  print(f"Sum of the elements in the major diagonal is {sum_diagonal}")

if __name__ == "__main__":
  main()