def reverse(number):
  tmpNumber = str(number)
  for i in range(len(tmpNumber) - 1, -1, -1):
    print(tmpNumber[i], end="")
  print()

if __name__ == "__main__":
  num = int(input("Enter an integer: "))
  reverse(num)