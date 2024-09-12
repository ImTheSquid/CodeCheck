def printJarakDalamMilDanKilo(jarak):
    for jarak in range(jarak, 11):
        print(f'{jarak}\t\t{jarak * 1.609:.3f}')

def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")
    jarak = 1
    printJarakDalamMilDanKilo(jarak)

if __name__ == "__main__":
    main()