from sheet_data import ATTRIBUTES, ATTRIBUTE_SKILLS

def generate_script():
    return "\n".join([
        '<script type="text/worker">',
        *[attribute_change(a.lower()) for a in ATTRIBUTES],
        *[attribute_skills(a.lower()) for a in ATTRIBUTE_SKILLS],
        armor_defense(),
        fortitude(),
        reflex(),
        mental(),
        threat(),
        encumbrance(),
        initiative(),
        base_speed(),
        legend_points(),
        '</script>',
        ""
    ])


def attribute_change(a):
    return f"""
        on("change:level change:{a}_starting change:{a}_misc", function(eventInfo) {{
            getAttrs(["{a}_starting", "{a}_misc", "level"], function(v) {{
                var starting = Number(v.{a}_starting);
                var scaling = 0;
                if (starting === 1) {{
                    scaling = Math.floor(v.level / 2);
                }} else if (starting > 1) {{
                    scaling = Number(v.level) - 1;
                }}
                setAttrs({{
                    {a}: starting + scaling + Number(v.{a}_misc),
                    {a}_scaling: scaling,
                }});
            }});
        }});
    """


def attribute_skills(attribute):
    return '\n'.join([
        set_skill(attribute, skill.lower().replace(' ', '_'))
        for skill in ATTRIBUTE_SKILLS[attribute]
    ])


def set_skill(a, s):
    if a == 'other':
        return f"""
            on("change:level change:{s}_points change:{s}_misc", function(eventInfo) {{
                getAttrs(["level", "{s}_points", "{s}_misc"], function(v) {{
                    var level = Number(v.level);
                    var pointsModifier = 0;
                    var ranks = 0;

                    if (Number(v.{s}_points) === 1) {{
                        ranks = Math.floor(level / 2);
                        pointsModifier = 2;
                    }} else if (Number(v.{s}_points) >= 2) {{
                        ranks = level
                        pointsModifier = 4;
                    }}

                    setAttrs({{
                        {s}: ranks + pointsModifier + Number(v.{s}_misc),
                        {s}_ranks: ranks,
                    }});
                }});
            }});
        """
    else:
        include_encumbrance = a in ['strength', 'dexterity']
        encumbrance_changed = ' change:encumbrance' if include_encumbrance else ""
        get_encumbrance = ', "encumbrance"' if include_encumbrance else ""
        subtract_encumbrance = ' - Number(v.encumbrance)' if include_encumbrance else ""
        return f"""
            on("sheet:opened change:level change:{a} change:{s}_points change:{s}_misc{encumbrance_changed}", function(eventInfo) {{
                getAttrs(["level", "{a}", "{s}_points", "{s}_misc"{get_encumbrance}], function(v) {{
                    var level = Number(v.level);
                    var pointsModifier = 0;
                    var ranks = 0;

                    if (Number(v.{s}_points) === 1) {{
                        ranks = Math.floor(level / 2);
                        pointsModifier = 2;
                    }} else if (Number(v.{s}_points) >= 2) {{
                        ranks = level
                        pointsModifier = 4;
                    }}

                    var scaling = Math.max(ranks, Number(v.{a}));

                    setAttrs({{
                        {s}: scaling + pointsModifier + Number(v.{s}_misc){subtract_encumbrance},
                        {s}_ranks: ranks,
                    }});
                }});
            }});
        """

def armor_defense():
    return f"""
        on("change:level change:dexterity change:body_armor_defense_value change:shield_defense_value change:armor_defense_misc", function(eventInfo) {{
            getAttrs(["level", "dexterity", "body_armor_defense_value", "shield_defense_value", "armor_defense_misc"], function(v) {{
                var scaling = Math.max(Number(v.level), Number(v.dexterity));
                var total = scaling + Number(v.body_armor_defense_value) + Number(v.shield_defense_value) + Number(v.armor_defense_misc);
                setAttrs({{
                    armor_defense: total,
                    armor_defense_scaling: scaling,
                }});
            }});
        }});
    """

def fortitude():
    return f"""
        on("change:level change:strength change:constitution change:fortitude_class change:fortitude_misc", function(eventInfo) {{
            getAttrs(["level", "strength", "constitution", "constitution_starting", "fortitude_class", "fortitude_misc"], function(v) {{
                var scaling = Math.max(Number(v.level), Number(v.strength), Number(v.constitution));
                var total = scaling + Number(v.constitution_starting) + Number(v.fortitude_class) + Number(v.fortitude_misc);
                setAttrs({{
                    fortitude: total,
                    fortitude_scaling: scaling,
                }});
            }});
        }});
    """

def reflex():
    return f"""
        on("change:level change:dexterity change:perception change:shield_defense_value change:reflex_class change:reflex_misc", function(eventInfo) {{
            getAttrs(["level", "dexterity", "perception", "dexterity_starting", "reflex_class", "reflex_misc"], function(v) {{
                var scaling = Math.max(Number(v.level), Number(v.dexterity), Number(v.perception));
                var total = scaling + Number(v.dexterity_starting) + Number(v.reflex_class) + Number(v.reflex_misc);
                setAttrs({{
                    reflex: total,
                    reflex_scaling: scaling,
                }});
            }});
        }});
    """

def mental():
    return f"""
        on("change:level change:intelligence change:willpower change:mental_class change:mental_misc", function(eventInfo) {{
            getAttrs(["level", "intelligence", "willpower", "willpower_starting", "mental_class", "mental_misc"], function(v) {{
                var scaling = Math.max(Number(v.level), Number(v.intelligence), Number(v.willpower));
                var total = scaling + Number(v.willpower_starting) + Number(v.mental_class) + Number(v.mental_misc);
                setAttrs({{
                    mental: total,
                    mental_scaling: scaling,
                }});
            }});
        }});
    """

def threat():
    return f"""
        on("change:level change:strength change:body_armor_defense_value change:threat_misc", function(eventInfo) {{
            getAttrs(["level", "strength", "body_armor_defense_value", "threat_misc"], function(v) {{
                var scaling = Math.max(Number(v.level), Number(v.strength));
                var armor_modifier = Math.floor(Number(v.body_armor_defense_value) / 2);
                setAttrs({{
                    threat: scaling + armor_modifier + Number(v.threat_misc),
                    threat_armor: armor_modifier,
                    threat_scaling: scaling,
                }});
            }});
        }});
    """

def encumbrance():
    return f"""
        on("change:body_armor_encumbrance change:encumbrance_misc change:constitution_starting", function(eventInfo) {{
            getAttrs(["body_armor_encumbrance", "encumbrance_misc", "constitution_starting"], function(v) {{
                setAttrs({{
                    encumbrance: Math.max(
                        Number(v.body_armor_encumbrance)
                        + Number(v.encumbrance_misc)
                        - Number(v.constitution_starting)
                    , 0),
                }});
            }});
        }});
    """

def initiative():
    return f"""
        on("change:dexterity change:perception change:initiative_misc", function(eventInfo) {{
            getAttrs(["dexterity", "perception", "initiative_misc"], function(v) {{
                var scaling = Math.max(Number(v.dexterity), Number(v.perception));
                setAttrs({{
                    initiative: scaling + Number(v.initiative_misc),
                    initiative_scaling: scaling,
                }});
            }});
        }});
    """

def base_speed():
    return f"""
        on("change:speed_size change:speed_armor change:speed_misc", function(eventInfo) {{
            getAttrs(["speed_size", "speed_armor", "speed_misc"], function(v) {{
                setAttrs({{
                    base_speed: Number(v.speed_size) - Number(v.speed_armor) + Number(v.speed_misc),
                }});
            }});
        }});
    """

def legend_points():
    return f"""
        on("change:level", function(eventInfo) {{
            getAttrs(["level"], function(v) {{
                setAttrs({{
                    legend_points: Number(v.level) >= 3 ? 1 : 0,
                }});
            }});
        }});
    """
