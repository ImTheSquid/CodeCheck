import numpy as np

# Input angka ke dalam array n x n pada bagian ini
arr = np.array([[1, 2, 3, 4],
               [5, 6, 7, 8],
               [9, 10, 11, 12],
               [13, 14, 15, 16]])

# Menjumlahkan angka yang terdapat di dalam array
sum = 0
for i in range(len(arr)):
    sum += arr[i][i]

# Menampilkan hasil dari pemanggilan fungsi sumMajorDiagonal
print("Sum of the elements in the major diagonal is " + str(sum))