def reverse(number):
  angka = str(number)
  for i in range(len(angka) - 1, -1, -1):
    print(angka[i], end="")

number = int(input("Enter an integer: "))
reverse(number)