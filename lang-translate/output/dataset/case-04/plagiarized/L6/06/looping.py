def print_miles_km(mil):
  for i in range(mil, 11):
    x = int(i * 1.609)
    print(f"{i}\t\t{x}")

print("Miles\t\tKilometers")
print("-------------------------------")
print_miles_km(1)