def Konversi(miles):
    return miles * 1.609

print("Miles\t\tKilometers")
print("-------------------------------")
# Use while loop
for miles in range(1, 11):
    print(miles, "\t\t", Konversi(miles))