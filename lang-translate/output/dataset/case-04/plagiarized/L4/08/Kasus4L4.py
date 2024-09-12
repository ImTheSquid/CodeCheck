def ConvertMilesToKilo(miles):
    return miles * 1.609

counter = 1
print("Miles\t\tKilometers")
print("-------------------------------")
while counter <= 10:
    print(counter, "\t\t", ConvertMilesToKilo(counter))
    counter += 1