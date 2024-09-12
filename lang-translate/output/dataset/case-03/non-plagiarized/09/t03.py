import math

weight = float(input("Enter weight in pounds : "))
feet = float(input("Enter feet : "))
inches = float(input("Enter inches : "))

height = feet * 12 + inches
bmi = weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))
print("BMI is " + str(bmi))

if bmi < 18.5:
    print("underweight")
elif bmi >= 18.5 and bmi < 25:
    print("normal")
elif bmi >= 25 and bmi < 35:
    print("overweight")
else:
    print("obese")