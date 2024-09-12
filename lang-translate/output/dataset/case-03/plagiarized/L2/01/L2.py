berat = 150
feet = 5
inci = 10
tinggi = feet * 12 + inci

bmi = berat * 0.45359237 / ((tinggi * 0.0254) * (tinggi * 0.0254))

print("BMI is", bmi)
if bmi < 18.5:
    print("Underweight")
elif bmi < 25:
    print("Normal")
elif bmi < 30:
    print("Overweight")
else:
    print("Obese")