def main():
  arr = []
  # Input angka ke dalam array n x n pada bagian ini
  print("Enter a 4 by 4 matrix row by row: ")
  for i in range(4):
    row = []
    for j in range(4):
      row.append(float(input()))
    arr.append(row)
  # Menjumlahkan angka yang terdapat di dalam array
  sum = 0
  for i in range(len(arr)):
    sum += arr[i][i]
  # Menampilkan hasil dari pemanggilan fungsi sumMajorDiagonal
  print("Sum of the elements in the major diagonal is " + str(sum))

if __name__ == "__main__":
  main()