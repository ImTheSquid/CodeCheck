# Miles to Kilometers Conversion Table

print("Miles\t\tKilometers")
print("-------------------------------")

miles = 1
while miles <= 10:
    kilometers = miles * 1.609
    print(f"{miles}\t\t{kilometers:.3f}")
    miles += 1