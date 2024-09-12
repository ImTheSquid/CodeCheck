def hitungBmi(weight, height):
    bmi = weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))
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
    kaki = float(input("Enter feet: "))
    inchi = float(input("Enter inches: "))
    tinggi = kaki * 12 + inchi
    hitungBmi(berat, tinggi)