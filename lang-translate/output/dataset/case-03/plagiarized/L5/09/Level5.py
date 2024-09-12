def bmi(weight, height):
    bmi = weight * 0.45359237 / Math.pow((height * 0.0254), 2)
    print(f"BMI is {bmi}")
    if bmi < 18.5:
        print("Underweight")
    elif bmi < 25:
        print("Normal")
    elif bmi < 30:
        print("Overweight")
    else:
        print("Obese")


if __name__ == "__main__":
    berat = float(input("Enter weight in pounds: "))
    feet = float(input("Enter feet: "))
    inches = float(input("Enter inches: "))
    tinggi = feet * 12 + inches
    bmi(berat, tinggi)
