def main():
    print("Miles\tKilometers")
    for i in range(1, 11):
        print(f'{i}\t{i * 1.609:.1f}')

if __name__ == "__main__":
    main()