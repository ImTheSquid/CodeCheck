def convert():
    for mill in range(1, 11):
        print(f'{mill}\t\t{mill * 1.609}')

def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")

    convert()

if __name__ == "__main__":
    main()