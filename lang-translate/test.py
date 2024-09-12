def main():
    # Prompt the user to enter weight in pounds
    weight = float(input("Enter weight in pounds: "))

    # Prompt the user to enter height
    feet = float(input("Enter feet: "))
    inches = float(input("Enter inches: "))

    height = feet * 12 + inches

    # Compute BMI
    bmi = weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))

    # Display result
    print("BMI is " + str(bmi))
    if bmi < 18.5:
        print("Underweight")
    elif bmi < 25:
        print("Normal")
    elif bmi < 30:
        print("Overweight")
    else:
        print("Obese")

if __name__ == '__main__':
    main()