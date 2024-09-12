def main():
    # Get weight in pounds
    weight = float(input("Enter weight in pounds: "))

    # Get height in feet and inches
    feet = float(input("Enter feet: "))
    inches = float(input("Enter inches: "))

    # Calculate height in inches
    height_inches = feet * 12 + inches

    # Calculate BMI
    bmi = weight * 0.45359237 / ((height_inches * 0.0254) * (height_inches * 0.0254))

    # Print BMI and weight category
    print(f"BMI is {bmi:.2f}")
    if bmi < 18.5:
        print("Underweight")
    elif bmi < 25:
        print("Normal")
    elif bmi < 30:
        print("Overweight")
    else:
        print("Obese")

if __name__ == "__main__":
    main()
