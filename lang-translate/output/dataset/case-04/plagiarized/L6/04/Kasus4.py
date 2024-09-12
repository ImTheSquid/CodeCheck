def Konversi(miles):
    if miles <= 10:
        print(miles, "\t\t", miles * 1.609)
        return Konversi(miles + 1)
    return 0

def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")
    Konversi(1)

if __name__ == "__main__":
    main()