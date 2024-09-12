bobot = float(input("Enter weight in pounds: "))
kaki = float(input("Enter feet: "))
inc = float(input("Enter inches: "))
height = kaki * 12 + inc
bmi = bobot * 0.45359237 / ((height * 0.0254) * (height * 0.0254))
print(f"BMI is {bmi}")
if bmi < 18.5:
  print("Underweight")
elif bmi < 25:
  print("Normal")
elif bmi < 30:
  print("Overweight")
else:
  print("Obese")