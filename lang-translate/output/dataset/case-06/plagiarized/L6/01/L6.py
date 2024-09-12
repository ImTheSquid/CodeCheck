def print_arr(arr):
  i = 9
  j = 0
  while j < 10:
    print(arr[i])
    i -= 1
    j += 1

def main():
  angka = [0] * 10
  i = 0
  while i != 10:
    angka[i] = int(input("Read a number: "))
    i += 1

  print_arr(angka)

if __name__ == "__main__":
  main()