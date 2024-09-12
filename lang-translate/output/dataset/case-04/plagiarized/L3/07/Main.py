def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")
    jarak = 1
    while jarak <= 10:
        print(jarak, "\t\t", jarak * 1.609)
        jarak += 1


if __name__ == "__main__":
    main()