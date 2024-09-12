def print_miles_to_km(mil):
    for i in range(mil, 11):
        print(f"{i}\t\t{i * 1.609:.3f}")

print("Miles\t\tKilometers")
print("-------------------------------")
print_miles_to_km(1)