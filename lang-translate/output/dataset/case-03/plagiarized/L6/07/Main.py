def hitungBmi(berat, height):
    bmi = berat * 0.45359237 / (Math.pow((height * 0.0254), 2))
    return bmi

berat = float(input("Enter weight in pounds: "))
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))
height = feet * 12 + inches
bmi = hitungBmi(berat, height)
print(f"BMI is {bmi}")

if bmi < 18.5:
    print("Underweight")
elif bmi < 25:
    print("Normal")
elif bmi < 30:
    print("Overweight")
else:
    print("Obese")