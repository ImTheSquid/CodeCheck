def main():
    w = float(input("Enter weight in pounds : "))
    f = float(input("Enter feet : "))
    i = float(input("Enter inches : "))
    h = f * 12 + i
    TotalBmi = w * 0.45359237 / ((h * 0.0254) * (h * 0.0254))
    print("BMI is " + str(TotalBmi))
    if TotalBmi < 18.5:
        print("Underweight ")
    elif TotalBmi < 25:
        print("Normal ")
    elif TotalBmi < 30:
        print("Overweight ")
    else:
        print("Obese ")

if __name__ == "__main__":
    main()