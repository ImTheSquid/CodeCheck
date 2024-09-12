# Miles to Kilometers Conversion Table

print("Miles\t\tKilometers")
print("-------------------------------")

# Loop for 10 iterations
for distance in range(1, 11):
    # Calculate kilometers
    kilometers = distance * 1.609
    # Print results
    print(f"{distance}\t\t{kilometers:.3f}") 