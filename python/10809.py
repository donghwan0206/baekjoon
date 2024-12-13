s = input()[:100]
r = ""
for k in range(ord("a"), ord("z") + 1):
    ol = len(s)
    ll = len(s.split(chr(k))[0])
    r += f"{ll if ol != ll else -1 } "
print(r)


# # s = input()[:100]
# r = ""
# for k in "abcdefghijklmnopqrstuvwxyz":
#     i = 0
#     found = False
#     for i, c in enumerate(s):
#         if c == k:
#             r += f"{i} "
#             found = True
#             break
#     if len(s) == 0 or not found:
#         r += f"-1 "
# print(r)
