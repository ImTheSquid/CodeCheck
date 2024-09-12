def ConvertMilesToKilo(miles):
    return miles * 1.609

print("Miles\t\tKilometers")
print("-------------------------------")
for i in range(1, 11):
    print(f"{i}\t\t{ConvertMilesToKilo(i)}")