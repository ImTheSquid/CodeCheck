def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")
    miles = 1
    while miles <= 10:
        print(f'{miles}\t\t{miles * 1.609}')
        miles += 1

if __name__ == '__main__':
    main()