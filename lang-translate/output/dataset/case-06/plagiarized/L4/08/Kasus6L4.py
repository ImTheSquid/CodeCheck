def Reverse(arr):
    for i in range(9, -1, -1):
        #Print angka dalam array
        print(arr[i])

if __name__ == '__main__':
    #Delarasi
    arrNum = [0] * 10
    # for untuk 10 kali input
    for i in range(10):
        # Read a number
        arrNum[i] = int(input("Read a number: "))
    # Display the array
    Reverse(arrNum)