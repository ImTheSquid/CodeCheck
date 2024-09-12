def main():
    # Masukan beban dalam satuan pounds
    berat = float(input("Enter weight in pounds: "))
    # Masukan tinggi dalam satuan kaki dan inci
    feet = float(input("Enter feet: "))
    inches = float(input("Enter inches: "))
    height = feet * 12 + inches
    # Hitung bmi di sini
    bmi = berat * 0.45359237 / ((height * 0.0254) * (height * 0.0254))
    # Print hasil di sini
    print("BMI is " + str(bmi))
    if bmi < 18.5:
        print("Underweight")
    elif bmi < 25:
        print("Normal")
    elif bmi < 30:
        print("Overweight")
    else:
        print("Obese")

if __name__ == "__main__":
    main()