def main():
  matriks = []
  sum = 0
  for b in range(4):
    row = []
    for k in range(4):
      row.append(float(input()))
    matriks.append(row)

  for x in range(len(matriks)):
    sum += matriks[x][x]

  print(f'Sum of the elements in the major diagonal is {sum}')

if __name__ == '__main__':
  main()