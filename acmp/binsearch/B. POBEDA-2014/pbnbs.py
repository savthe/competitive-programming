a, b, c, d = [int(x) for x in input().split()]

n = min(a, b) + min(c, d)

print(int(n**0.5 + 0.0000001))
