def main():
    berat = float(input("Enter weight in pounds: "))
    kaki = float(input("Enter feet: "))
    inch = float(input("Enter inches: "))
    tinggi = kaki * 12 + inch

    rata = berat * 0.45359237 /((tinggi * 0.0254) * (tinggi * 0.0254))

    print(f"BMI is {rata}")
    if rata < 18.5:
        print("Underweight")
    elif rata < 25:
        print("Normal")
    elif rata < 30:
        print("Overweight")
    else:
        print("Obese")

if __name__ == "__main__":
    main()