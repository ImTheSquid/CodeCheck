def reverse(number):
  balik = ""
  while number > 0:
    balik = balik + str(number % 10)
    number = number // 10
  print(balik)

number = int(input("Enter an integer: "))
reverse(number)