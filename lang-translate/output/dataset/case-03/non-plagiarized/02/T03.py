def main():
    w = float(input("Enter weight in pounds: "))
    f = float(input("Enter feet: "))
    i = float(input("Enter inches: "))
    h = f * 12 + i
    bmi = (w * 0.45359237) / ((h * 0.0254) * (h * 0.0254))
    print(f"BMI is {bmi:.2f}")
    if bmi < 18.5:
        print("Underweight")
    elif 18.5 <= bmi < 25:
        print("Normal")
    elif 25 <= bmi < 35:
        print("Overweight")
    elif bmi >= 35:
        print("Obese")

if __name__ == "__main__":
    main()