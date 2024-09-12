def main():
    weight = float(input("Enter weight in pounds : "))
    feet = float(input("Enter feet : "))
    inches = float(input("Enter inches : "))

    height = feet * 12 + inches
    bmi = weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))
    print("BMI is " + str(bmi))
    if bmi < 18.5:
        print("Underweight")
    elif bmi >= 18.5 and bmi < 25:
        print("Normal")
    elif bmi >= 25 and bmi < 35:
        print("Overweight")
    else:
        print("obese")

if __name__ == "__main__":
    main()