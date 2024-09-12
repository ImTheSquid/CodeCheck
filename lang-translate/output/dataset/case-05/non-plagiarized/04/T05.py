def reverse(number):
  cek = str(number)
  for i in range(len(cek)):
    print(cek[len(cek) - 1 - i], end="")
  print()

number = int(input("Enter an integer "))
reverse(number)