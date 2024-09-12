import sys

def main():
    num = []

    for i in range(10):
        # Read a number
        num.append(int(input(f"Read a number: ")))

    # Display the array
    for i in range(9, -1, -1):
        print(num[i])

if __name__ == "__main__":
    main()