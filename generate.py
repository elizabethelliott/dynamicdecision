PARTICIPANTS = 400

CONDITIONS = [
    'dynamic',
    'dichotomous'
]

print("participants:")

for i in range(1, PARTICIPANTS+1):
    is_counterbalance = "true" if int(((i-1) / 50) % 2) > 0 else "false"
    condition = CONDITIONS[(i-1) % 2]
    print(f'  {i}:\n    counterbalance: {is_counterbalance}\n    condition: "{condition}"')

