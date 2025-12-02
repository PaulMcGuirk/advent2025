import re
import sys

def to_range(pc):
    bounds = [int(b) for b in pc.split('-')]
    return range(bounds[0], bounds[1] + 1)

if __name__ == "__main__":
    filename = sys.argv[1]

    input = None
    with open(filename,'r') as f:
        input = f.read().strip()

    ids = [id for pc in input.split(',') for id in to_range(pc)]

    part_one = sum(id for id in ids if re.match(r"^(\d+)\1$", str(id)))
    part_two = sum(id for id in ids if re.match(r"^(\d+)\1+$", str(id)))

    
    print(part_one)
    print(part_two)