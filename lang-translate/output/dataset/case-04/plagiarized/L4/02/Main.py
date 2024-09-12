def printMil(mil):
    while mil <= 10:
        print(f'{mil}\t\t{mil * 1.609}')
        mil += 1

#declare MIL
mil = 1
#Miles to kilometer
print("Miles\t\tKilometers")
print("-------------------------------")

# Use while loop
printMil(mil)