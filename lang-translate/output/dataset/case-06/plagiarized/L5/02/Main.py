def print_number(number):
  for i in range(9, -1, -1):
    print(number[i])

def main():
  number = [0] * 10
  i = 0
  while i < 10:
    number[i] = int(input("Read a number: "))
    i += 1
  print_number(number)

if __name__ == "__main__":
  main()