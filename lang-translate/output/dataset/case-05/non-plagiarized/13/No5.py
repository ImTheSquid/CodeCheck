def reverse(a):
  for i in range(len(a) - 1, -1, -1):
    print(a[i], end="")

inp = input("Enter an integer: ")
reverse(inp)