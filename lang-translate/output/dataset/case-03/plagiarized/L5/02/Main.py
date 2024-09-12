def countHeightUser(feet, inches):
  return feet * 12 + inches

def countBmi(height, weight):
  return weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254))

def printBmi(bmi):
  if bmi < 18.5:
    print("Underweight")
  elif bmi < 25:
    print("Normal")
  elif bmi < 30:
    print("Overweight")
  else:
    print("Obese")

# enter inches
inchesUser = float(input("Enter inches: "))

# Prompt the user to enter weight in pounds
weightUser = float(input("Enter weight in pounds: "))

# Prompt the user to enter height
# enter feet
feetUser = float(input("Enter feet: "))

# process
heightUser = countHeightUser(feetUser, inchesUser)
# Compute BMI
bmi = countBmi(heightUser, weightUser)
# Display result
print(f"BMI is {bmi}")
printBmi(bmi)