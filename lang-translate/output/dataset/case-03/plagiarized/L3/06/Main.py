# This code will calculate the Body Mass Index (BMI) of a person based on their weight in pounds and height in feet and inches.
# It will then print the BMI and categorize the weight based on the BMI value.

# Get the weight in pounds from the user.
weight = float(input("Enter weight in pounds: "))

# Get the height in feet and inches from the user.
feet = float(input("Enter feet: "))
inches = float(input("Enter inches: "))

# Calculate the total height in inches.
height = (feet * 12) + inches

# Convert height to meters.
height = height * 0.0254

# Calculate the BMI.
bmi = weight * 0.45359237 / (height * height)

# Print the BMI.
print("BMI is", bmi)

# Determine and print the weight category based on the BMI.
if bmi < 18.5:
    print("Underweight")
elif bmi < 25:
    print("Normal")
elif bmi < 30:
    print("Overweight")
else:
    print("Obese")
