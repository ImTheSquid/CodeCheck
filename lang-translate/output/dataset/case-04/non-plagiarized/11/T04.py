import decimal

def main():
    """
    Prints a table of miles and kilometers for miles 1-10.
    """
    print("Miles\tKilometers")
    for i in range(1, 11):
        kilometers = decimal.Decimal(i) * decimal.Decimal("1.609")
        print(f"{i}\t{kilometers:.3f}")

if __name__ == "__main__":
    main()