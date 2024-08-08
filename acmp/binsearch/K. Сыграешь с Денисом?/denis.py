# WA
import sys
n = 1
r = 0
while True:
    print('?', n)
    sys.stdout.flush();
    rep = int(input())
    if rep == 0 or rep <= r: break
    r = rep
    n *= 2

n -= rep
print('!', n)



