from sheet_data import ATTRIBUTES, ATTRIBUTE_SKILLS

def generate_script():
    return "\n".join([
        '<script type="text/worker">',
        *[attribute_change(a.lower()) for a in ATTRIBUTES],
        *[attribute_skills(a.lower()) for a in ATTRIBUTE_SKILLS],
        *core_statistics(),
        *defenses(),
        *resistances(),
        *abilities_known(),
        action_points(),
        level_scaling(),
        skill_points_spent(),
        '</script>',
        ""
    ])

def js_wrapper(variables, function_body):
    # not everything actually depends on level, but it's convenient to make
    # everything recalculate when level changes
    variables_with_level = sorted(list(set(variables + ['level'])))
    change_string = ' '.join([f'change:{var}' for var in variables_with_level])
    get_attrs_string = ', '.join([f'"{var}"' for var in variables])
    set_variables_string = ';\n    '.join([f'var {stringify_variable_name(var)} = Number(v["{var}"] || 0)' for var in variables]) + ';'
    return f"""
        on("{change_string}", function(eventInfo) {{
            getAttrs([{get_attrs_string}], function(v) {{
                {set_variables_string}
                {function_body}
            }});
        }});
    """

def stringify_variable_name(varname):
    return varname.replace('$', '')

def get_misc_variables(variable_name, count):
    return [f'{variable_name}_misc_{i}' for i in range(count)]

def sum_variables(variables):
    return '+'.join(variables)

def attribute_change(a):
    misc = get_misc_variables(a, 1)
    return js_wrapper(
        ['level', f'{a}_starting', *misc],
        f"""
            var scaling = 0;
            if ({a}_starting === 1) {{
                scaling = Math.floor(level / 2);
            }} else if ({a}_starting > 1) {{
                scaling = level - 1;
            }}
            setAttrs({{
                {a}: {a}_starting + scaling + {sum_variables(misc)},
                {a}_scaling: scaling,
            }});
        """
    )

def attribute_skills(attribute):
    return '\n'.join([
        set_skill(attribute, skill.lower().replace(' ', '_'))
        for skill in ATTRIBUTE_SKILLS[attribute]
    ])


def set_skill(a, s):
    misc = get_misc_variables(s, 3)
    if a == 'other':
        return js_wrapper(
            ['level', f'{s}_points', 'fatigue_penalty', *misc],
            f"""
                var pointsModifier = 0;
                var ranks = 0;
                var training = '';

                if ({s}_points === 1) {{
                    ranks = Math.floor(level / 2);
                    pointsModifier = 1;
                    training = 'T';
                }} else if (v.{s}_points >= 2) {{
                    ranks = level
                    pointsModifier = 3;
                    training = 'M';
                }}

                setAttrs({{
                    {s}_attribute: 0,
                    {s}_ranks: ranks + pointsModifier,
                    {s}_total: ranks + pointsModifier + {sum_variables(misc)} - fatigue_penalty,
                    {s}_training: training,
                }});
            """
        )
    else:
        include_encumbrance = a in ['strength', 'dexterity']
        subtract_encumbrance = ' - encumbrance' if include_encumbrance else ""
        return js_wrapper(
            ['level', f'{a}_starting', f'{s}_points', 'fatigue_penalty', *misc, *(['encumbrance'] if include_encumbrance else [])],
            f"""
                var pointsModifier = 0;
                var ranks = 0;
                var training = '';

                if ({a}_starting >= 2) {{
                    {s}_points += 1
                }}

                if ({s}_points === 1) {{
                    ranks = Math.floor(level / 2) + 1;
                    training = 'T';
                }} else if ({s}_points >= 2) {{
                    ranks = level
                    pointsModifier = 3;
                    training = 'M';
                }}

                setAttrs({{
                    {s}_ranks: ranks,
                    {s}_total: ranks + pointsModifier + {a}_starting + {sum_variables(misc)} - fatigue_penalty {subtract_encumbrance},
                    {s}_training: training,
                }});
            """
        )

def core_statistics():
    return [
        accuracy(),
        base_speed(),
        encumbrance(),
        fatigue_penalty(),
        fatigue_tolerance(),
        focus_penalty(),
        hit_points(),
        initiative(),
        insight_points(),
        magical_power(),
        mundane_power(),
        skill_points(),
    ]

def defenses():
    return [
        armor_defense(),
        fortitude(),
        mental(),
        reflex(),
    ]

def resistances():
    return [
        base_bleed_resistance(),
        base_vital_resistance(),
        all_resistance(),
        energy_resistance(),
        physical_resistance(),
        bleed_resistances(),
        vital_resistances(),
    ]

def abilities_known():
    return [
        maneuvers_known(),
        spells_known(),
        spheres_known(),
        *[blank_ability_known(i) for i in range(1)]
    ]

def accuracy():
    misc = get_misc_variables('accuracy', 2)
    return js_wrapper(
        ['challenge_rating', 'level', 'perception_starting', 'fatigue_penalty', *misc],
        f"""
            var cr_mod = challenge_rating === 0 ? 0 : Math.max(0, challenge_rating - 1);
            setAttrs({{
                accuracy: level + Math.floor(perception_starting / 2)  + {sum_variables(misc)} + cr_mod - fatigue_penalty,
            }});
        """
    )


def action_points():
    return js_wrapper(
        ['level'],
        f"""
            var action_points = 3 + Math.floor(level / 3);
            setAttrs({{
                action_points,
                action_points_max: action_points,
            }});
        """,
    )


# This only applies to monsters, which is defined as creatures with CR !== 0
def level_scaling():
    return js_wrapper(
        ['challenge_rating', 'level'],
        f"""
            var level_scaling = challenge_rating === 0 ? 0 : Math.floor((level + 1) / 6);
            setAttrs({{
                level_scaling,
            }});
        """,
    )


def armor_defense():
    misc = get_misc_variables('armor_defense', 1)
    return js_wrapper(
        ['level', 'dexterity_starting', 'armor_defense_class_bonus', 'body_armor_defense_value', 'shield_defense_value', *misc, 'challenge_rating'],
        f"""
            var cr_mod = challenge_rating === 0 ? 0 : Math.max(0, challenge_rating - 1);
            var before_equipment = level + dexterity_starting + cr_mod + armor_defense_class_bonus;
            var total = before_equipment + body_armor_defense_value + shield_defense_value + {sum_variables(misc)};
            setAttrs({{
                armor_defense: total,
            }});
        """
    )

def fortitude():
    misc = get_misc_variables('fortitude', 3)
    return js_wrapper(
        ['level', 'constitution_starting', 'fortitude_class', 'challenge_rating', *misc],
        f"""
            var cr_mod = challenge_rating === 0 ? 0 : Math.max(0, challenge_rating - 1);
            setAttrs({{
                fortitude: level + constitution_starting + fortitude_class + cr_mod + {sum_variables(misc)},
            }});
        """
    )

def reflex():
    misc = get_misc_variables('reflex', 3)
    return js_wrapper(
        ['level', 'dexterity_starting', 'reflex_class', 'challenge_rating', *misc],
        f"""
            var cr_mod = challenge_rating === 0 ? 0 : Math.max(0, challenge_rating - 1);
            setAttrs({{
                reflex: level + dexterity_starting + reflex_class + cr_mod + {sum_variables(misc)},
            }});
        """
    )

def mental():
    misc = get_misc_variables('mental', 3)
    return js_wrapper(
        ['level', 'willpower_starting', 'mental_class', 'challenge_rating', *misc],
        f"""
            var cr_mod = challenge_rating === 0 ? 0 : Math.max(0, challenge_rating - 1);
            setAttrs({{
                mental: level + willpower_starting + mental_class + cr_mod + {sum_variables(misc)},
            }});
        """
    )

def encumbrance():
    misc = get_misc_variables('encumbrance', 2)
    return js_wrapper(
        ['level', 'body_armor_encumbrance', 'strength_starting', *misc],
        f"""
            setAttrs({{
                encumbrance: Math.max(
                    body_armor_encumbrance
                    - Math.max(0, strength_starting)
                    - {'-'.join(misc)}
                , 0),
            }});
        """,
    )

def initiative():
    misc = get_misc_variables('initiative', 3)
    return js_wrapper(
        ['dexterity', 'perception', 'fatigue_penalty', *misc],
        f"""
            var scaling = Math.max(dexterity, perception);
            setAttrs({{
                initiative: scaling + {sum_variables(misc)} - fatigue_penalty,
                initiative_scaling: scaling,
            }});
        """
    )

def base_speed():
    misc = get_misc_variables('speed', 2)
    return js_wrapper(
        ['level', 'speed_size', 'speed_armor', *misc],
        f"""
            setAttrs({{
                base_speed: speed_size - speed_armor + {sum_variables(misc)}
            }});
        """
    )

def skill_points():
    misc = get_misc_variables('skill_points', 2)
    return js_wrapper(
        ['level', 'intelligence_starting', *misc],
        f"""
            setAttrs({{
                skill_points: 6 + intelligence_starting * 2 + {sum_variables(misc)}
            }});
        """
    )

def fatigue_tolerance():
    misc = get_misc_variables('fatigue_tolerance', 2)
    return js_wrapper(
        ['level', 'constitution_starting', *misc],
        f"""
            setAttrs({{
                fatigue_tolerance: 1 + constitution_starting + {sum_variables(misc)},
            }});
        """
    )


def focus_penalty():
    misc = get_misc_variables('focus_penalty', 3)
    return js_wrapper(
        ['level', *misc],
        f"""
            setAttrs({{
                focus_penalty: 4 - ({sum_variables(misc)}),
            }});
        """
    )


def hit_points():
    misc = get_misc_variables('hit_points', 2)
    return js_wrapper(
        ['constitution_starting', 'challenge_rating', *misc],
        f"""
            var hit_points = 9 + constitution_starting + {sum_variables(misc)};
            var cr_mod = {{
                0.5: 0,
                1: 0.5,
                2: 1,
                3: 2,
                4: 4,
            }}[challenge_rating] || 1;
            hit_points = Math.floor(hit_points * cr_mod)
            setAttrs({{
                hit_points,
                hit_points_max: hit_points,
            }});
        """
    )

def fatigue_penalty():
    return js_wrapper(
        ['fatigue_points', 'fatigue_tolerance'],
        f"""
            setAttrs({{
                fatigue_penalty: Math.max(0, fatigue_points - fatigue_tolerance),
            }});
        """
    )


def insight_points():
    misc = get_misc_variables('insight_points', 2)
    return js_wrapper(
        ['intelligence_starting', *misc],
        f"""
            setAttrs({{
                insight_points: 2 + intelligence_starting + {sum_variables(misc)}
            }});
        """
    )

def magical_power():
    misc = get_misc_variables('magical_power', 2)
    return js_wrapper(
        ['level', 'willpower_starting', 'challenge_rating', 'level_scaling', *misc],
        f"""
            var cr_mod = challenge_rating === 0 ? 0 : Math.max(0, challenge_rating - 1);
            setAttrs({{
                magical_power: level + willpower_starting + cr_mod + level_scaling * 2 + {sum_variables(misc)},
            }});
        """
    )

def mundane_power():
    misc = get_misc_variables('mundane_power', 2)
    return js_wrapper(
        ['level', 'strength_starting', 'challenge_rating', 'level_scaling', *misc],
        f"""
            var cr_mod = challenge_rating === 0 ? 0 : Math.max(0, challenge_rating - 1);
            setAttrs({{
                mundane_power: level + strength_starting + cr_mod + level_scaling * 2 + {sum_variables(misc)},
            }});
        """
    )

def maneuvers_known():
    misc = get_misc_variables('maneuvers_known', 2)
    return js_wrapper(
        ['maneuvers_known_base', 'maneuvers_known_insight_points', *misc],
        f"""
            setAttrs({{
                maneuvers_known: maneuvers_known_base + maneuvers_known_insight_points + {sum_variables(misc)}
            }});
        """
    )

def spheres_known():
    misc = get_misc_variables('spheres_known', 2)
    return js_wrapper(
        ['spheres_known_base', 'spheres_known_insight_points', *misc],
        f"""
            setAttrs({{
                spheres_known: spheres_known_base + Math.floor(spheres_known_insight_points / 2) + {sum_variables(misc)}
            }});
        """
    )

def spells_known():
    misc = get_misc_variables('spells_known', 2)
    return js_wrapper(
        ['spells_known_base', 'spells_known_insight_points', *misc],
        f"""
            setAttrs({{
                spells_known: spells_known_base + spells_known_insight_points + {sum_variables(misc)}
            }});
        """
    )

def blank_ability_known(i):
    name = f'blank_ability_known_{i}'
    misc = get_misc_variables(name, 2)
    return js_wrapper(
        [f'{name}_base', f'{name}_insight_points', *misc],
        f"""
            setAttrs({{
                {name}: {name}_base + {name}_insight_points + {sum_variables(misc)}
            }});
        """
    )

def base_bleed_resistance():
    return js_wrapper(
        ['level', 'constitution_starting'],
        f"""
            var base_bleed_resistance = {{
                '-1': 1 ,
                0:    1 ,
                1:    2 ,
                2:    3 ,
                3:    4 ,
                4:    5 ,
                5:    6 ,
                6:    7 ,
                7:    8 ,
                8:    10,
                9:    12,
                10:   14,
                11:   17,
                12:   20,
                13:   23,
                14:   26,
                15:   29,
                16:   32,
                17:   36,
                18:   40,
                19:   45,
                20:   50,
                21:   55,
                22:   60,
                23:   65,
                24:   70,
                25:   75,
            }}[level + constitution_starting] || 1;
            setAttrs({{
                base_bleed_resistance,
            }});
        """
    )

def base_vital_resistance():
    return js_wrapper(
        ['level', 'constitution', 'constitution_starting'],
        f"""
            var base_vital_resistance = {{
                '-2':   10 ,
                '-1':   11 ,
                0:      12 ,
                1:      13 ,
                2:      15 ,
                3:      17 ,
                4:      19 ,
                5:      23 ,
                6:      27 ,
                7:      32 ,
                8:      37 ,
                9:      43 ,
                10:     49 ,
                11:     55 ,
                12:     61 ,
                13:     68 ,
                14:     75 ,
                15:     82 ,
                16:     90 ,
                17:     100,
                18:     110,
                19:     120,
                20:     130,
                21:     140,
                22:     150,
                23:     160,
                24:     170,
                25:     180,
            }}[level + constitution_starting] || 10;
            setAttrs({{
                base_vital_resistance,
            }});
        """
    )

def all_resistance():
    misc = get_misc_variables('all_resistance_bonus', 4)
    return js_wrapper(
        misc,
        f"""
            setAttrs({{
                all_resistance_bonus: {sum_variables(misc)},
            }});
        """
    )

def energy_resistance():
    misc = get_misc_variables('energy_resistance_bonus', 3)
    return js_wrapper(
        ['energy_resistance_bonus_armor', *misc],
        f"""
            setAttrs({{
                energy_resistance_bonus: energy_resistance_bonus_armor + {sum_variables(misc)},
            }});
        """
    )

def physical_resistance():
    misc = get_misc_variables('physical_resistance_bonus', 3)
    return js_wrapper(
        ['physical_resistance_bonus_armor', *misc],
        f"""
            setAttrs({{
                physical_resistance_bonus: physical_resistance_bonus_armor + {sum_variables(misc)},
            }});
        """
    )

def bleed_resistances():
    return js_wrapper(
        ['base_bleed_resistance', 'all_resistance_bonus', 'energy_resistance_bonus', 'physical_resistance_bonus', 'challenge_rating'],
        f"""
            var global_bleed_resistance = base_bleed_resistance + all_resistance_bonus;
            var resistance_modifier = challenge_rating === 0.5 ? 0.5 : 1;
            setAttrs({{
                global_bleed_resistance: Math.floor(global_bleed_resistance * resistance_modifier),
                energy_bleed_resistance: Math.floor((global_bleed_resistance + energy_resistance_bonus) * resistance_modifier),
                physical_bleed_resistance: Math.floor((global_bleed_resistance + physical_resistance_bonus) * resistance_modifier),
            }})
        """
    )

def vital_resistances():
    return js_wrapper(
        ['base_vital_resistance', 'all_resistance_bonus', 'energy_resistance_bonus', 'physical_resistance_bonus', 'challenge_rating'],
        f"""
            var global_vital_resistance = base_vital_resistance + all_resistance_bonus;
            var resistance_modifier = challenge_rating === 0.5 ? 0.5 : 1;
            setAttrs({{
                global_vital_resistance: Math.floor(global_vital_resistance * resistance_modifier),
                energy_vital_resistance: Math.floor((global_vital_resistance + energy_resistance_bonus) * resistance_modifier),
                physical_vital_resistance: Math.floor((global_vital_resistance + physical_resistance_bonus) * resistance_modifier),
            }})
        """
    )

def skill_points_spent():
    skill_names = [skill_name for skills in ATTRIBUTE_SKILLS.values() for skill_name in skills]
    skill_names = [skill_name.lower().replace(' ', '_') for skill_name in skill_names]
    skill_points_names = [f"{skill_name}_points" for skill_name in skill_names]

    return js_wrapper(
        skill_points_names,
        f"""
            var skill_points_spent = {' + '.join(skill_points_names)};

            setAttrs({{
                skill_points_spent,
            }});
        """
    )

def standard_damage_at_power(power):
    return {
        '-2': '1d4',
        0: '1d6',
        2: '1d8',
        4: '1d10',
        6: '2d6',
        8: '2d8',
        10: '2d10',
        12: '4d6',
        14: '4d8',
        16: '4d10',
        18: '5d10',
        20: '6d10',
        22: '7d10',
        24: '8d10',
    }[power]
