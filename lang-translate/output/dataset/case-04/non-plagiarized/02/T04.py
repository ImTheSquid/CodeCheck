def main():
    print("Miles       Kilometers")
    for i in range(1, 11):
        if i == 10:
            print(f"{i}          {i * 1609}")
        else:
            print(f"{i}           {i * 1609}")

if __name__ == "__main__":
    main()