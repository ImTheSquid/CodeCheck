def fungsiBmi(berat, feet, inches):
    tinggi = feet * 12 + inches
    return berat * 0.45359237 / Math.pow((tinggi * 0.0254), 2)

def result(res):
    print("BMI is " + str(res))
    if res < 18.5:
        print("Underweight")
    elif res >= 18.5 and res < 25:
        print("Normal")
    elif res >= 25 and res < 30:
        print("Overweight")
    else:
        print("Obese")

berat = float(input("Enter weight in pounds: "))
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))

result(fungsiBmi(berat, feet, inches))