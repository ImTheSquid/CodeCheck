def main():
    matrix = []
    for _ in range(4):
        row = [float(x) for x in input().split()]
        matrix.append(row)

    hasil = 0
    for i in range(4):
        hasil += matrix[i][i]

    print(f"Sum of the elements in the major  diagonal is  {hasil}")

if __name__ == "__main__":
    main()