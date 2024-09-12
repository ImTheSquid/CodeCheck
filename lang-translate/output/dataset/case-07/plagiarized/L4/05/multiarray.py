import numpy as np

def sum(m):
    """
    Adds all elements in the major diagonal of a 2D array.

    Args:
        m (list): A 2D array.

    Returns:
        float: The sum of the elements in the major diagonal.
    """
    sum = 0
    for i in range(len(m)):
        sum += m[i][i]
    return sum

# 25779F8829AB7A7650E85A4CC871E6AC sangat ganteng
if __name__ == "__main__":
    print("Enter a 4 by 4 matrix row by row: ")
    ma = []
    for i in range(4):
        row = [float(x) for x in input().split()]
        ma.append(row)
    
    sum = np.trace(np.array(ma))
    
    print(f"Sum of the elements in the major diagonal is {sum}")
