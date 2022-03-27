from cgi_simple import (
    button,
    checkbox,
    div,
    equation,
    fieldset,
    flex_col,
    flex_row,
    flex_wrapper,
    label,
    labeled_span,
    labeled_text_input,
    number_input,
    option,
    plus,
    select,
    sidelabel,
    span,
    textarea,
    this_or_that,
    underlabel,
    underlabeled_checkbox,
    underlabel_spaced,
)


def create_page(_destination):
    return flex_col(
        {"class": "page status-page"},
        [
            div(
                {"class": "tab-explanation"},
                """
                This tab is used to track changes to your character's statistics.
                All standard debuffs and circumstances are listed here.
                In addition, you can define custom modifiers to represent your character's various abilities.
            """,
            ),
            flex_row(
                {"class": "standard-modifiers"},
                [
                    flex_col(
                        [
                            div({"class": "section-header"}, "Circumstances"),
                            circumstances(),
                        ]
                    ),
                    flex_col(
                        [
                            div({"class": "section-header"}, "Debuffs"),
                            debuffs(),
                        ]
                    ),
                    flex_col(
                        {"class": "vital-wounds"},
                        [
                            div({"class": "section-header"}, "Vital Wounds"),
                            flex_row(
                                {"class": "vital-roll-row"},
                                [
                                    button(
                                        {
                                            "type": "roll",
                                            "value": "@{character_name} makes a vital roll: [[d10+@{vital_rolls}]]",
                                        },
                                        "Roll vital wound",
                                    ),
                                    underlabel(
                                        "Roll mod",
                                        number_input(
                                            {
                                                "disabled": True,
                                                "name": "vital_rolls_display",
                                                "value": "@{vital_rolls}",
                                            }
                                        ),
                                    ),
                                ],
                            ),
                            fieldset(
                                {"class": f"repeating_vitalwounds"},
                                vital_wound(),
                            ),
                        ],
                    ),
                ],
            ),
            flex_wrapper(div({"class": "section-header"}, "Temporary Modifiers")),
            fieldset(
                {"class": "repeating_temporarymodifiers"},
                custom_modifier(show_toggle=True),
            ),
            flex_wrapper(div({"class": "section-header"}, "Permanent Modifiers")),
            fieldset(
                {"class": "repeating_permanentmodifiers"},
                custom_modifier(show_toggle=False),
            ),
            textarea(
                {
                    "class": "hidden",
                    "disabled": True,
                    "name": "debuff_headers",
                    "value": "",
                }
            ),
        ],
    )


def vital_wound():
    return flex_row(
        {"class": "vital-wound"},
        [
            underlabel_spaced(
                "Vital Roll",
                number_input({"class": "fake-text", "name": "vital_wound_roll"}),
                {"class": "vital-wound-roll"},
            ),
            labeled_span(
                "Effect",
                {"class": "vital-wound-effect"},
                {
                    "name": "vital_wound_effect",
                },
            ),
        ],
    )


def circumstances():
    return flex_row(
        {"class": "debuffs"},
        [
            flex_col(
                [
                    debuff("climbing"),
                    debuff("flying"),
                    debuff("flying poorly"),
                    debuff("grappled"),
                    debuff("helpless"),
                ]
            ),
            flex_col(
                [
                    debuff("partially unaware"),
                    debuff("prone"),
                    debuff("squeezing"),
                    debuff("swimming"),
                    debuff("unaware"),
                ]
            ),
        ],
    )


def debuffs():
    return flex_row(
        {"class": "debuffs"},
        [
            flex_col(
                [
                    debuff("blinded"),
                    debuff("confused"),
                    debuff("dazed"),
                    debuff("dazzled"),
                    debuff("frightened"),
                    debuff("goaded"),
                ]
            ),
            flex_col(
                [
                    debuff("immobilized"),
                    debuff("panicked"),
                    debuff("shaken"),
                    debuff("slowed"),
                    debuff("stunned"),
                ]
            ),
        ],
    )


def debuff(name, representable=True):
    return label(
        {"class": "debuff-active-label"},
        [
            checkbox({"name": name.replace(" ", "_")}),
            span(
                {
                    "class": "representable-debuff"
                    if representable
                    else "unrepresentable-debuff"
                },
                name.capitalize() + "?",
            ),
        ],
    )


def custom_modifier(show_toggle):
    return (
        flex_row(
            {"class": "custom-modifier"},
            [
                (
                    underlabeled_checkbox(
                        "Active?",
                        None,
                        {"class": "is-active", "name": "is_active"},
                    )
                    if show_toggle else ""
                ),
                labeled_text_input(
                    "Name",
                    {"class": "name"},
                    {"name": "name"},
                ),
                "".join([custom_statistic(str(i)) for i in range(0, 4)]),
            ],
        ),
    )


def custom_statistic(i):
    return flex_row(
        [
            select(
                {"name": "statistic" + i},
                [
                    option({"value": ""}, ""),
                    option({"value": "accuracy"}, "Accuracy"),
                    option({"value": "all_defenses"}, "All defenses"),
                    option({"value": "all_skills"}, "All skills"),
                    option({"value": "armor_defense"}, "Armor defense"),
                    option({"value": "attunement_points"}, "Attunement points"),
                    option({"value": "awareness"}, "Awareness"),
                    option({"value": "balance"}, "Balance"),
                    option({"value": "climb"}, "Climb"),
                    option({"value": "constitution"}, "Constitution"),
                    option({"value": "craft (alchemy)"}, "Craft (alchemy)"),
                    option({"value": "craft (bone)"}, "Craft (bone)"),
                    option({"value": "craft (ceramics)"}, "Craft (ceramics)"),
                    option({"value": "craft (jewelry)"}, "Craft (jewelry)"),
                    option({"value": "craft (leather)"}, "Craft (leather)"),
                    option({"value": "craft (manuscripts)"}, "Craft (manuscripts)"),
                    option({"value": "craft (metal)"}, "Craft (metal)"),
                    option({"value": "craft (poison)"}, "Craft (poison)"),
                    option({"value": "craft (stone)"}, "Craft (stone)"),
                    option({"value": "craft (textiles)"}, "Craft (textiles)"),
                    option({"value": "craft (traps)"}, "Craft (traps)"),
                    option({"value": "craft (wood)"}, "Craft (wood)"),
                    option({"value": "creature handling"}, "Creature Handling"),
                    option({"value": "damage_resistance_bonus"}, "Damage resist"),
                    option({"value": "deception"}, "Deception"),
                    option({"value": "deduction"}, "Deduction"),
                    option({"value": "devices"}, "Devices"),
                    option({"value": "dexterity"}, "Dexterity"),
                    option({"value": "disguise"}, "Disguise"),
                    option({"value": "encumbrance"}, "Encumbrance"),
                    option({"value": "endurance"}, "Endurance"),
                    option({"value": "fatigue_tolerance"}, "Fatigue tolerance"),
                    option({"value": "flexibility"}, "Flexibility"),
                    option({"value": "fortitude"}, "Fortitude defense"),
                    option({"value": "hit_points"}, "Hit points"),
                    option({"value": "insight_points"}, "Insight points"),
                    option({"value": "intelligence"}, "Intelligence"),
                    option({"value": "intimidate"}, "Intimidate"),
                    option({"value": "jump"}, "Jump"),
                    option({"value": "knowledge (arcana)"}, "Knowledge (arcana)"),
                    option({"value": "knowledge (dungeoneering)"}, "Knowledge (dungeoneering)"),
                    option({"value": "knowledge (engineering)"}, "Knowledge (engineering)"),
                    option({"value": "knowledge (items)"}, "Knowledge (items)"),
                    option({"value": "knowledge (local)"}, "Knowledge (local)"),
                    option({"value": "knowledge (nature)"}, "Knowledge (nature)"),
                    option({"value": "knowledge (planes)"}, "Knowledge (planes)"),
                    option({"value": "knowledge (religion)"}, "Knowledge (religion)"),
                    option({"value": "linguistics"}, "Linguistics"),
                    option({"value": "medicine"}, "Medicine"),
                    option({"value": "mental"}, "Mental defense"),
                    option({"value": "none"}, ""),
                    option({"value": "perception"}, "Perception"),
                    option({"value": "perform"}, "Perform"),
                    option({"value": "persuasion"}, "Persuasion"),
                    option({"value": "power"}, "Power"),
                    option({"value": "profession"}, "Profession"),
                    option({"value": "reflex"}, "Reflex defense"),
                    option({"value": "ride"}, "Ride"),
                    option({"value": "sleight of hand"}, "Sleight of Hand"),
                    option({"value": "social insight"}, "Social Insight"),
                    option({"value": "speed"}, "Speed"),
                    option({"value": "stealth"}, "Stealth"),
                    option({"value": "strength"}, "Strength"),
                    option({"value": "weapon_damage_dice"}, "Strike +d damage"),
                    option({"value": "survival"}, "Survival"),
                    option({"value": "swim"}, "Swim"),
                    option({"value": "nonclass_skill_count"}, "Trained skills"),
                    option({"value": "vital_rolls"}, "Vital rolls"),
                    option({"value": "willpower"}, "Willpower"),
                ],
            ),
            underlabel(
                "Value",
                number_input({"name": "value" + i}),
                {"class": "custom-modifier-value"},
            ),
        ]
    )
