arr = []
for x in range(10):
  arr.append(int(input(f"Read a number: ")))

for x in range(9, -1, -1):
  print(arr[x])