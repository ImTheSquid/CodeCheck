nums = [0] * 10

for i in range(10, -1, -1):
    nums[i] = int(input(f"Read a number: "))

for i in range(9, -1, -1):
    print(nums[i])