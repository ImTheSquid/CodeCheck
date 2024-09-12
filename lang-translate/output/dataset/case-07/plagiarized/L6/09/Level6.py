def ngitung(matriks):
    jumlah = 0
    i = 3
    while i >= 0:
        jumlah += matriks[i][i]
        i -= 1
    print("Sum of the elements in the major diagonal is " + str(jumlah))

def main():
    mat = []
    print("Enter a 4 by 4 matrix row by row: ")
    for baris in range(4):
        row = []
        for kolom in range(4):
            row.append(float(input()))
        mat.append(row)
    ngitung(mat)

if __name__ == "__main__":
    main()