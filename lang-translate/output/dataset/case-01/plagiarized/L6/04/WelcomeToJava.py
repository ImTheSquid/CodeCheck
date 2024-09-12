def test(i):
    if i > 0:
        print("Welcome To Java")
        return test(i - 1)
    else:
        return ""


if __name__ == "__main__":
    i = 5
    test(i)