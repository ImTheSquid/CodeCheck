def terbalik(angka):
  arr = []
  while angka > 0:
    sisa = angka % 10
    arr.append(sisa)
    angka = angka // 10
  for i in range(len(arr) - 1, -1, -1):
    print(arr[i], end="")
  print()

input = int(input("Enter an integer: "))
terbalik(input)