def cetak(bmi):
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

def main():
    weights = float(input("Enter weight in pounds: "))
    feets = float(input("Enter feet: "))
    inches = float(input("Enter inches: "))

    #Compute BMI
    heights = feets * 12 + inches
    bmi = weights * 0.45359237 / ((heights * 0.0254) * (heights * 0.0254))
    cetak(bmi)

if __name__ == "__main__":
    main()