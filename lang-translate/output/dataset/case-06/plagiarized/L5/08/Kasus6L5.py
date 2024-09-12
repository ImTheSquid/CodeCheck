def Reverse(arr):
    i = 9
    while i >= 0:
        #Print angka dalam array
        print(arr[i])
        i -= 1

def main():
    #Delarasi
    arrNum = [0] * 10
    # for untuk 10 kali input
    i = 0
    while i < 10:
        # Read a number
        arrNum[i] = int(input("Read a number: "))
        i += 1
    # Display the array
    Reverse(arrNum)

if __name__ == "__main__":
    main()