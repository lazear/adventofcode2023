from functools import reduce
# Time:        54     94     65     92
# Distance:   302   1476   1029   1404

# xs = [(54, 302), (94, 1476), (65, 1029), (92, 1404)]
# xs = [(54, 302), (94, 1476), (65, 1029), (92, 1404)]
xs= [(54946592, 302147610291404)]

acc = 1
for (time, dist) in xs:
    wins = 0
    for hold in range(time):
        distance = hold * (time - hold)
        if distance > dist:
            wins += 1
    acc *= wins
print(acc)
        
            # wins.append(hold)
    # print(reduce(lambda x,y: x * y, wins, 1))
    
    
