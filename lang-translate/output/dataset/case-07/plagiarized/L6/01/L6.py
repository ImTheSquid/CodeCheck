def jumlahDiagonal(m):
    sum = 0
    hit = len(m) - 1

    while hit >= 0:
        sum += m[hit][hit]
        hit -= 1
    return sum

def print(m):
    print(f"Sum of the elements in the major diagonal is {jumlahDiagonal(m)}")

if __name__ == '__main__':
    m = []
    print("Enter a 4 by 4 matrix row by row: ")

    for i in range(4):
        row = []
        for j in range(4):
            row.append(float(input()))
        m.append(row)
    
    print(m)