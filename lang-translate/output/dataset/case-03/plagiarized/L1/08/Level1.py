def calculate_bmi(weight, height):
  """Calculates BMI given weight in pounds and height in inches."""
  bmi = weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))
  return bmi

def get_bmi_category(bmi):
  """Returns the BMI category based on the calculated BMI."""
  if bmi < 18.5:
    return "Underweight"
  elif bmi < 25:
    return "Normal"
  elif bmi < 30:
    return "Overweight"
  else:
    return "Obese"

# Get user input
weight = float(input("Enter weight in pounds: "))
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))
height = feet * 12 + inches

# Calculate BMI
bmi = calculate_bmi(weight, height)

# Print results
print("BMI is", bmi)
print(get_bmi_category(bmi))