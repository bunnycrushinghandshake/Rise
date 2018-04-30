from cgi_simple import value_sum

ATTRIBUTES = 'Strength Dexterity Constitution Intelligence Perception Willpower'.split()
DEFENSES = 'Armor Fortitude Reflex Mental'.split()
ATTRIBUTE_SKILLS = {
    'strength': 'Climb Jump Swim'.split(),
    'dexterity': ['Acrobatics', 'Escape Artist', 'Ride', 'Sleight of Hand', 'Stealth'],
    'constitution': [],
    'intelligence': ['Craft ______', 'Deduction', 'Devices', 'Disguise', 'Heal', 'Knowledge _____', 'Knowledge ______', 'Linguistics'],
    'perception': ['Awareness', 'Creature Handling', 'Sense Motive', 'Spellcraft', 'Survival'],
    'willpower': [],
    'other': ['Bluff', 'Intimidate', 'Perform ______', 'Persuasion'],
}

ALL_SKILLS = ['Awareness', 'Balance', 'Bluff', 'Climb', 'Craft', 'Creature Handling',
              'Devices', 'Disguise', 'Escape Artist', 'Heal', 'Intimidate', 'Jump',
              'Knowledge', 'Linguistics', 'Perform', 'Persuasion',
              'Ride', 'Sense Motive', 'Sleight of Hand', 'Spellcraft', 'Sprint',
              'Stealth', 'Survival', 'Swim', 'Tumble']

ROLL20_CALC = {
    'action_points': value_sum([
        'action_points_base',
        'action_points_willpower',
        'action_points_misc',
    ]),
    'armor_defense': value_sum([
        'armor_scaling',
        'body_armor_defense_value',
        'shield_defense_value',
        'armor_misc',
    ]),
    'base_speed': value_sum([
        'speed_size',
        'speed_armor',
        'speed_misc',
    ]),
    # Odd syntax since we're subtracting constitution
    'encumbrance': value_sum([
        'body_armor_encumbrance',
        'encumbrance_misc',
    ]) + ' - @{constitution_starting})',
    'hit_points': '(@{hit_points_misc} + (@{hit_points_level} * @{hit_points_constitution}))',
    'skill_points': value_sum([
        'skill_points_class',
        'skill_points_intelligence',
        'skill_points_misc',
    ]),
    'strike_accuracy': value_sum([
        'strike_accuracy_scaling',
        'strike_accuracy_misc',
    ]),
}
