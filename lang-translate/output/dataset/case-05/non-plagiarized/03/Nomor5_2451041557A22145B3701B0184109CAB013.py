def reverse(number):
    strNumber = str(number)
    temp = ""
    for i in range(len(strNumber), 0, -1):
        temp += str(i)
    tempNumber = int(temp)
    print(tempNumber)

number = int(input("Enter an integer : "))
reverse(number)