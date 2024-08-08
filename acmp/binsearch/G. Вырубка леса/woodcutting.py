def test(a, b, k, m, x, days):
    s = a * days - a * (days // k); 
    s += b * days - b * (days // m); 
    return s >= x

a, k, b, m, x = [int(x) for x in input().split()]

low = 0
high = 10**18

while low < high:
    days = (low + high) // 2
    if not test(a, b, k, m, x, days): 
        low = days + 1
    else:
        high = days

print(low)
