r = int(input())
for i in range(r):
    n, t = input().split(" ")
    n = int(n)
    for c in t:
        print(c * n, end="")
    print()
