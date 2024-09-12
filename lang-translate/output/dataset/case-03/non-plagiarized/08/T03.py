def calculate_bmi(weight, height):
    """Calculates the BMI based on weight in pounds and height in inches.

    Args:
        weight (float): Weight in pounds.
        height (float): Height in inches.

    Returns:
        float: Body Mass Index (BMI)
    """
    bmi = weight * 0.45359237 / Math.pow((height * 0.0254), 2)
    return bmi

def get_bmi_category(bmi):
    """Determines the BMI category based on the calculated BMI.

    Args:
        bmi (float): Body Mass Index (BMI).

    Returns:
        str: BMI category (underweight, normal, overweight, obese)
    """
    if bmi < 18.5:
        return "underweight"
    elif bmi < 25:
        return "normal"
    elif bmi < 35:
        return "overweight"
    else:
        return "obese"

# Test the code with sample values
weight = 150
feet = 5
inches = 10

height = feet * 12 + inches

bmi = calculate_bmi(weight, height)
bmi_category = get_bmi_category(bmi)

print(f"BMI is {bmi:.2f}")
print(f"BMI Category: {bmi_category}")