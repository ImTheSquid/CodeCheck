def printMil(mil):
    for i in range(1, 11):
        print(f'{mil}\t\t{mil * 1.609:.3f}')
        mil += 1

if __name__ == "__main__":
    mil = 1
    print("Miles\t\tKilometers")
    print("-------------------------------")
    printMil(mil)