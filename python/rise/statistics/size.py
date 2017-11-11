# For brevity, return a three-item array instead of a dict.
# The order is [Fort, Ref, strike damage, space, reach, speed].
def properties_from_size_name(name):
    return {
        'fine': [8, -4, 1, 1, 10],
        'diminuitive': [6, -3, 1, 1, 15],
        'tiny': [4, -2, 2.5, 2.5, 20],
        'small': [2, -1, 5, 5, 25],
        'medium': [0, 0, 0, 5, 5, 30],
        'large': [-2, 1, 10, 10, 40],
        'huge': [-4, 2, 15, 15, 50],
        'gargantuan': [-6, 3, 20, 20, 60],
        # Should the space/reach here be 25?
        'colossal': [-8, 4, 30, 30, 70],
    }[name]

class Size(object):
    MEDIUM = 'medium'

    def __init__(self, name):
        self.name = name
        properties = properties_from_size_name(name)
        self.reflex_defense_modifier = properties[0]
        self.damage_modifier = properties[1]
        self.space = properties[2]
        self.reach = properties[3]
        self.speed = properties[4]
