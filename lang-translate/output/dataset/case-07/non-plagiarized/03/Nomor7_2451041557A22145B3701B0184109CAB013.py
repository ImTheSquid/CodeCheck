def main():
    matrix = []
    sum = 0
    print("Enter a 4-by-4 matrix row by row: ")
    for i in range(4):
        row = [int(x) for x in input().split()]
        matrix.append(row)
        for j in range(4):
            if i == j:
                sum += matrix[i][j]
    print("Sum of the elements in the major diagonal is", sum)

if __name__ == "__main__":
    main()