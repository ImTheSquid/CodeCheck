arr = []
for i in range(10):
  arr.append(int(input(f"Read a number: ")))

for i in range(9, -1, -1):
  print(arr[i])