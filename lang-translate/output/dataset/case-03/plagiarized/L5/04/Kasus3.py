def hitungTinggi(feet, inches):
    return feet * 12 + inches

def hitungBmi(weight, height):
    return weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))

def cekBmi(bmi):
    if bmi < 18.5:
        print("Underweight")
    elif bmi < 25:
        print("Normal")
    elif bmi < 30:
        print("Overweight")
    else:
        print("Obese")

# Prompt the user to enter weight in pounds
weight = float(input("Enter weight in pounds: "))
# Prompt the user to enter height 
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))
# Compute BMI
bmi = hitungBmi(weight, hitungTinggi(feet, inches))
# Display result
print("BMI is " + str(bmi))
cekBmi(bmi)