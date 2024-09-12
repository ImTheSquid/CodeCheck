def hitBmi(berat, tinggi):
  return berat * 0.45359237 / ((tinggi * 0.0254) * (tinggi * 0.0254))

berat = float(input("Enter weight in pounds: "))
feet = float(input("Enter feet: "))
inci = float(input("Enter inches: "))
tinggi = feet * 12 + inci

print("BMI is " + str(hitBmi(berat, tinggi)))
if hitBmi(berat, tinggi) < 25 and hitBmi(berat, tinggi) > 18.5:
  print("Normal")
elif hitBmi(berat, tinggi) < 30 and hitBmi(berat, tinggi) > 25:
  print("Overweight")
elif hitBmi(berat, tinggi) > 30:
  print("Obese")
else:
  print("Underweight")