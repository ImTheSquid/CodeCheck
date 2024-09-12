def HitBMI(w, h):
    return w * 0.45359237 / ((h * 0.0254) * (h * 0.0254))

def Result(TotalBmi):
    if TotalBmi < 18.5:
        print("Underweight ")
    elif TotalBmi < 25:
        print("Normal ")
    elif TotalBmi < 30:
        print("Overweight ")
    else:
        print("Obese ")

w = float(input("Enter weight in pounds : "))
f = float(input("Enter feet : "))
i = float(input("Enter inches : "))
h = f * 12 + i
TotalBmi = HitBMI(w, h)
print("BMI is " + str(TotalBmi))
Result(TotalBmi)