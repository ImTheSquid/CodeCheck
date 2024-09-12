def jumlahDiagonal(m):
  sum = 0
  hit = 0
  while hit != len(m):
    sum += m[hit][hit]
    hit += 1
  return sum

def print(m):
  print(f"Sum of the elements in the major diagonal is {jumlahDiagonal(m)}")

m = [[0 for _ in range(4)] for _ in range(4)]
print("Enter a 4 by 4 matrix row by row: ")
for i in range(4):
  for j in range(4):
    m[i][j] = float(input())

print(m)