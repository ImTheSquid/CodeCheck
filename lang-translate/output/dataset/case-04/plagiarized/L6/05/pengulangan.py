def Konversi(n):
    miles = 1
    while miles <= n:
        print(miles, "\t\t", miles * 1.609)
        miles += 1

print("Miles\t\tKilometers")
print("-------------------------------")
# Use while loop
Konversi(10)