def konversi(mile):
    if mile != 11:
        print(mile, "\t\t", mile * 1.609)
        mile += 1
        konversi(mile)

if __name__ == "__main__":
    mile = 1
    print("Miles\t\tKilometers")
    print("-------------------------------")
    konversi(mile)