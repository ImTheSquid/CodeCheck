berat = 180
kaki = 5
inch = 10

tinggi = kaki * 12 + inch
rata = berat * 0.45359237 /((tinggi * 0.0254) * (tinggi * 0.0254))

print("BMI is " + str(rata))
if (rata < 18.5):
    print("Underweight")
elif (rata < 25):
    print("Normal")
elif (rata < 30):
    print("Overweight")
else:
    print("Obese")