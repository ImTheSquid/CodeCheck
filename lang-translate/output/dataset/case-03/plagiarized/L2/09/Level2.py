def calculate_bmi(berat, tinggi):
  """Calculates BMI given weight in pounds and height in inches."""
  bmi = berat * 0.45359237 / ((tinggi * 0.0254) * (tinggi * 0.0254))
  return bmi

def print_bmi_category(bmi):
  """Prints the BMI category based on the given BMI value."""
  if bmi < 18.5:
    print("Underweight")
  elif bmi < 25:
    print("Normal")
  elif bmi < 30:
    print("Overweight")
  else:
    print("Obese")

# Get input from the user
berat = float(input("Enter weight in pounds: "))
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))

# Calculate height in inches
tinggi = feet * 12 + inches

# Calculate BMI
bmi = calculate_bmi(berat, tinggi)

# Print BMI and category
print("BMI is", bmi)
print_bmi_category(bmi)