class inputoutput:
    kaki, bobot, bmi, inc, height = 0, 0, 0, 0, 0

    def hitHeight(kaki, inc):
        return kaki * 12 + inc

    def hitBmi(bobot, height):
        return bobot * 0.45359237 / ((height * 0.0254) * (height * 0.0254))

    def main():
        bobot = float(input("Enter weight in pounds: "))
        kaki = float(input("Enter feet: "))
        inc = float(input("Enter inches: "))
        height = inputoutput.hitHeight(kaki, inc)
        bmi = inputoutput.hitBmi(bobot, height)

        print("BMI is " + str(bmi) + "\n")
        if bmi >= 30:
            print("Obese")
        elif bmi >= 25:
            print("Overweight")
        elif bmi >= 18.5:
            print("Normal")
        else:
            print("Underweight" + "\n")


if __name__ == "__main__":
    inputoutput.main()