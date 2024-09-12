def main():
    mile = 1
    print("Miles\t\tKilometers")
    print("-------------------------------")
    while mile <= 10:
        print(mile, "\t\t", mile * 1.609)
        mile += 1


if __name__ == "__main__":
    main()
