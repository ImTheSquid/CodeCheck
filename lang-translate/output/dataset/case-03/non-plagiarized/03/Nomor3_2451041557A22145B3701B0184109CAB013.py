def main():
    weight = int(input("Enter weight in pounds: "))
    feet = int(input("Enter feet: "))
    inches = int(input("Enter inch: "))
    height = feet * 12 + inches
    bmi = weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))
    print("BMI is", bmi)
    if bmi < 18.5:
        print("underweight")
    elif bmi >= 18.5 and bmi < 25:
        print("Normal")
    elif bmi >= 25 and bmi < 35:
        print("overweight")
    elif bmi > 35:
        print("obese")


if __name__ == "__main__":
    main()