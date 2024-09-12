def hitHasil(m):
  for a in range(m, 11):
    print(f'{a}\t\t{a * 1.609:.3f}')

if __name__ == "__main__":
  m = 1
  print("Miles\t\tKilometers")
  print("-------------------------------")
  hitHasil(m)