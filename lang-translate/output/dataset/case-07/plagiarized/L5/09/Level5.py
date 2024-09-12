import numpy as np

def sum(x):
  jumlah = 0
  i = 0
  while i < 4:
    jumlah += x[i][i]
    i += 1
  return jumlah

def hasil(x):
  print("Sum of the elements in the major diagonal is "+ str(sum(x)))

matriks = np.zeros((4,4))
print("Enter a 4 by 4 matrix row by row: ")
for bar in range(4):
  for kol in range(4):
    matriks[bar][kol] = float(input())

hasil(matriks)