def forward(arr):
    res = []
    for i in range(product(arr[:-2])):
        idx = tuple(getIndex(arr[:-2], i))
        res[idx] = hash(i)

    return res


def getIndex(dims, num):
    total = product(dims)
    if num >= total:
        return None
    
    res = []
    
    for i in dims:
        total //= i
        res.append(num // total)
        num %= total
        
    return res
    
def product(arr):
    res = 1
    for i in arr:
        res *= i
    return res

a = [3, 2, 2, 4]
for i in range(49):
    
    print(getIndex(a, i)) 
    
print(forward(a))