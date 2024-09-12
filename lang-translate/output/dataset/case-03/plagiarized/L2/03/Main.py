def main():
    #Prompt the user to enter weight in pounds 
    weights = float(input("Enter weight in pounds: "))
    
    #Prompt the user to enter height
    feets = float(input("Enter feet: "))
    inches = float(input("Enter inches: "))
    
    heights = feets * 12 + inches
    
    #Compute BMI
    bmi = weights * 0.45359237 / ((heights * 0.0254) * (heights * 0.0254))
    
    #Display result
    print("BMI is " + str(bmi))
    if (bmi < 18.5):
        print("Underweight")
    elif (bmi < 25):
        print("Normal")
    elif (bmi < 30):
        print("Overweight")
    else:
        print("Obese")

if __name__ == "__main__":
    main()