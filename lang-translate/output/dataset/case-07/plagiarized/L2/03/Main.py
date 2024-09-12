def hasilPertambahanDiagonal(m):
  sum = 0
  for i in range(len(m)):
    sum += m[i][i]
  return sum

if __name__ == "__main__":
  n = []
  for i in range(4):
    n.append([float(x) for x in input(f"Enter row {i+1}: ").split()])
  print(f"Sum of the elements in the major diagonal is {hasilPertambahanDiagonal(n)}")