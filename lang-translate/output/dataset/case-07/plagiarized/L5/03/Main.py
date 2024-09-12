def main():
    n = []
    for i in range(4):
        row = []
        for j in range(4):
            row.append(float(input()))
        n.append(row)

    sum = 0
    for i in range(4):
        sum += n[i][i]

    print(f"Sum of the elements in the major diagonal is {sum}")

if __name__ == "__main__":
    main()