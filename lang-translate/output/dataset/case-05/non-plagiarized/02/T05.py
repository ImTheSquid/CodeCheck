def reverse(number):
  temp = str(number)
  for i in range(len(temp), 0, -1):
    print(temp[i-1], end="")

number = int(input("Enter a integer: "))
reverse(number)