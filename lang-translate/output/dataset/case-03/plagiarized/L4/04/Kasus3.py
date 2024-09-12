def hitungTinggi(feet, inches):
    return feet * 12 + inches

def hitungBmi(weight, feet, inches):
    return weight * 0.45359237 / ((hitungTinggi(feet, inches) * 0.0254) * (hitungTinggi(feet, inches) * 0.0254))

# Prompt the user to enter weight in pounds
weight = float(input("Enter weight in pounds: "))
# Prompt the user to enter height
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))
# Compute BMI
bmi = hitungBmi(weight, feet, inches)
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
