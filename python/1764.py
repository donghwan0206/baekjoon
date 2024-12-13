n, m = map(int, input().split(" "))
db = set()
bb = set()
for i in range(n):
    db.add(input())
for i in range(m):
    bb.add(input())

inter = list(db.intersection(bb))
inter.sort()
print(len(inter))
for name in inter:
    print(name)
