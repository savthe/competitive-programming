w, h, n = [int(x) for x in input().split()]
a = 1 
b = max(w*n, h*n)

while a < b:
	m = (a + b) // 2
	if (m//w)*(m//h) >= n: b = m
	else: a = m + 1

print(a)
