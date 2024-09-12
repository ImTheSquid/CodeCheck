def rekursif(bil):
    if bil != 0:
        sisaMod = bil % 10
        print(sisaMod, end='')
        bil = bil // 10
        rekursif(bil)
    return bil

bil = int(input("Enter an integer: "))
rekursif(bil)
print("")