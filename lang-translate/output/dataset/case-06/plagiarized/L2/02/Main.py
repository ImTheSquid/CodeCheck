number = []

# Input the number
for i in range(10):
  number.append(int(input("Read a number: ")))

# Output the number
for i in range(9, -1, -1):
  print(number[i])
