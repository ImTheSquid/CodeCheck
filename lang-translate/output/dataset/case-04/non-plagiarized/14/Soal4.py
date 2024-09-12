import decimal

def main():
    for i in range(11):
        if i == 0:
            print("Miles \t Kilometers")
        else:
            print(f"{i} \t {decimal.Decimal(i * 1.609).quantize(decimal.Decimal('0.001'))}")

if __name__ == "__main__":
    main()