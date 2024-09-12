def hitungBMI(weight, height):
  return weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))

def BMIKategori(BMI):
  print("BMI is " + str(BMI))
  if (BMI < 18.5):
    print("Underweight")
  elif (BMI < 25):
    print("Normal")
  elif (BMI < 30):
    print("Overweight")
  else:
    print("Obese")

# Inputting data
berat = float(input("Enter weight in pounds: "))
kaki = float(input("Enter feet: "))
inch = float(input("Enter inches: "))

BMIKategori(hitungBMI(berat, kaki * 12 + inch))
