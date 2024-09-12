def Konversi():
    for miles in range(1, 11):
        print(miles, "\t\t", miles * 1.609)

def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")
    Konversi()

if __name__ == "__main__":
    main()