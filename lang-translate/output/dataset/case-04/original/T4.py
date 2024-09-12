def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")

    # Use while loop
    miles = 1
    while miles <= 10:
        print(miles, "\t\t", miles * 1.609)
        miles += 1

if __name__ == "__main__":
    main()