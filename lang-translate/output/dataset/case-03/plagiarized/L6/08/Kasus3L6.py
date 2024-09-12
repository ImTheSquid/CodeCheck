import math

def hitungBMI(weight, height):
    temp = height * 0.0254
    return weight * 0.45359237 / (temp * temp)

def BMIKategori(BMI):
    print("BMI is " + str(BMI))
    if (BMI > 30):
        print("Obese")
    elif (BMI > 25):
        print("Overweight")
    elif (BMI > 18.5):
        print("Normal")
    else:
        print("Underweight")

berat = float(input("Enter weight in pounds: "))
kaki = float(input("Enter feet: "))
inch = float(input("Enter inches: "))

BMIKategori(hitungBMI(berat, kaki * 12 + inch))