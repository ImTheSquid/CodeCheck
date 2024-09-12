def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")

    mil = 1
    while mil <= 10:
        print(mil, "\t\t", mil * 1.609)
        mil += 1

if __name__ == "__main__":
    main()