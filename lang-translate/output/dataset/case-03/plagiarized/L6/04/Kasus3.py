def hitungTinggi(feet, inches):
    return feet * 12 + inches

def hitungBmi(weight, feet, inches):
    return weight * 0.45359237 / ((hitungTinggi(feet, inches) * 0.0254) * (hitungTinggi(feet, inches) * 0.0254))

def cekBmi(bmi):
    if bmi < 18.5:
        return "Underweight"
    elif bmi < 25:
        return "Normal"
    elif bmi < 30:
        return "Overweight"
    else:
        return "Obese"

# Prompt the user to enter weight in pounds
weight = float(input("Enter weight in pounds: "))
# Prompt the user to enter height 
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))
# Display result
print("BMI is " + str(hitungBmi(weight, feet, inches)))
print(cekBmi(hitungBmi(weight, feet, inches)))