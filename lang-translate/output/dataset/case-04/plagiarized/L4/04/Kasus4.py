def Konversi(miles):
    print(f'{miles}\t\t{miles * 1.609}')

def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")
    # Use while loop
    for miles in range(1, 11):
        Konversi(miles)

if __name__ == "__main__":
    main()