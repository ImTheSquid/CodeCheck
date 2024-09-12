def print_miles_kilometers(mil):
    for i in range(10, 0, -1):
        print(f"{mil}\t\t{mil * 1.609:.3f}")
        mil += 1

def main():
    mil = 1
    print("Miles\t\tKilometers")
    print("-------------------------------")
    print_miles_kilometers(mil)

if __name__ == "__main__":
    main()