def main():
    nums = []
    for i in range(10):
        nums.append(int(input(f"Read a number: ")))

    for i in range(9, -1, -1):
        print(nums[i])

if __name__ == "__main__":
    main()