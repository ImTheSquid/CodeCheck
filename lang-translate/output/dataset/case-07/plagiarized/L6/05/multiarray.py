import numpy as np

def sum(m):
  sum = 0
  for i in range(len(m)):
    sum += m[i][i]
  return sum

def main():
  ma = np.zeros((4, 4))
  print("Enter a 4 by 4 matrix row by row: ")
  for i in range(4):
    for j in range(4):
      ma[i][j] = float(input())
  sum = 1
  for i in range(1, len(ma)):
    sum += ma[i][i]
  print("Sum of the elements in the major diagonal is", sum - 1)

if __name__ == "__main__":
  main()