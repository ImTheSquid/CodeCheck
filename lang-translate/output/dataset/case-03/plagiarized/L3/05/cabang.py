weight = float(input("Enter weight in pounds: "))
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))

height = feet * 12 + inches
height_meters = height * 0.0254

bmi = weight * 0.45359237 / (height_meters * height_meters)
print("BMI is", bmi)

if bmi < 18.5:
    print("Underweight")
elif bmi < 25:
    print("Normal")
elif bmi < 30:
    print("Overweight")
else:
    print("Obese")
