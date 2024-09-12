w = int(input("Enter weight in pounds: "))
f = int(input("Enter feet: "))
i = int(input("Enter inches: "))
bmi = ((w * 0.45359237) / (((f * 12 + i) * 0.0254) * ((f * 12 + i) * 0.0254)))
print(f"BMI is {bmi}")
if bmi < 18.5:
    print("Underweight")
elif bmi >= 18.5 and bmi < 25:
    print("Normal")
elif bmi >= 25 and bmi < 35:
    print("Overweight")
elif bmi >= 35:
    print("Obese")