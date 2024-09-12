def Reverse(arr):
    i = 0
    while i < 10:
        print(arr[i])
        i += 1

def main():
    arrNum = [0] * 10
    for i in range(9, -1, -1):
        arrNum[i] = int(input("Read a number: "))
    Reverse(arrNum)

if __name__ == "__main__":
    main()