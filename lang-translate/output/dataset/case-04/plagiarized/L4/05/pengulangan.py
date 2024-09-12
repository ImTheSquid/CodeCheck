def hitHasil(m):
  while m <= 10:
    print(f'{m}\t\t{m * 1.609:.3f}')
    m += 1

if __name__ == "__main__":
  m = 1
  print("Miles\t\tKilometers")
  print("-------------------------------")
  hitHasil(m)