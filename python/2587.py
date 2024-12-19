int_list = []
for i in range(5):
    k = int(input())
    flag = False
    for j in range(i):
        if k > int_list[j]:
            continue
        elif k <= int_list[j]:
            flag = True
            break

    if i == 0 or not flag:
        int_list.append(k)
    elif flag:
        int_list.insert(j, k)

mid = int_list[len(int_list) // 2]
avg = sum(int_list) // len(int_list)
print(avg)
print(mid)
