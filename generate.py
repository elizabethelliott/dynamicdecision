PARTICIPANTS = 400

CONDITIONS = [
    'throughout',
    'final'
]

print("participants:")

for i in range(1, PARTICIPANTS+1):
    #is_counterbalance = "true" if int(((i-1) / 50) % 2) > 0 else "false"
    is_counterbalance = "false"
    condition = CONDITIONS[(i-1) % len(CONDITIONS)]
    print(f'  {i}:\n    counterbalance: {is_counterbalance}\n    condition: "{condition}"')

