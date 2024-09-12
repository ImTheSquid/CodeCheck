def diagonal(x):
  jumlah = 0
  for i in range(len(x)):
    jumlah += x[i][i]
  return jumlah

matriks = []
print("Enter a 4 by 4 matrix row by row: ")
for _ in range(4):
  row = [float(x) for x in input().split()]
  matriks.append(row)

print(f'Sum of the elements in the major diagonal is {diagonal(matriks)}')
