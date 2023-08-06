use crate::core_mechanics::Attribute;
use crate::equipment::Apparel::{Belt, Cloak};
use crate::equipment::{Apparel, ItemUpgrade, StandardItem};

pub fn torso() -> Vec<Apparel> {
    let mut apparel = vec![];

    apparel.append(&mut belts());
    apparel.append(&mut cloaks());

    apparel
}

// Life and healing effects
// Full body modification effects
fn belts() -> Vec<Apparel> {
    let mut apparel = vec![];

    apparel.push(Belt(StandardItem {
        name: String::from("Belt of Regeneration"),
        rank: 3,
        short_description: String::from("Regain 1d8 hit points while below half hit points"),
        description: String::from(r"
            At the end of each round, you regain 1d8 \glossterm{hit points}.
            This healing cannot increase your hit points above half your maximum hit points.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Regain 2d8 hit points while below half hit points", "
                The healing increases to 2d8.
            "),
            ItemUpgrade::new(7, "Regain 4d8 hit points while below half hit points", "
                The healing increases to 4d8.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Belt of Vital Regeneration"),
        rank: 5,
        short_description: String::from("Automatically exert to remove vital wounds"),
        description: String::from(r"
            At the end of each round, if your \glossterm<fatigue level> does not exceed your \glossterm<fatigue tolerance>, you automatically remove one of your \glossterm<vital wounds>.
            You can choose to stop this regeneration if you are conscious, but it happens automatically if you are unconscious due to vital wounds.
            When you remove a vital wound in this way, you increase your \glossterm<fatigue level> by three.
        "),
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Poisonbane Belt"),
        rank: 2,
        short_description: String::from("Impervious to poisons"),
        description: String::from(r"
            You are \impervious to \glossterm{poison}.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Immune to form-altering attacks", r"
                You become immune instead of impervious.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Belt of Health"),
        rank: 2,
        short_description: String::from("Grants +4 hit points"),
        description: String::from(r"
            You gain a +4 \glossterm<magic bonus> to your \glossterm<hit points>.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Grants +8 hit points", "
                The bonus increases to +8.
            "),
            ItemUpgrade::new(7, "Grants +16 hit points", "
                The bonus increases to +16.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Lifekeeping Belt"),
        rank: 1,
        short_description: String::from("Grants +1 to vital rolls"),
        description: String::from(r"
            You gain a +1 \glossterm<magic bonus> to your \glossterm<vital rolls>.
        "),
        upgrades: vec![
            ItemUpgrade::new(3, "Grants +2 to vital rolls", "
                The bonus increases to +2.
            "),
            ItemUpgrade::new(5, "Grants +3 to vital rolls", "
                The bonus increases to +3.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Healing Belt"),
        rank: 2,
        short_description: String::from("Exert to heal $dr3 hit points"),
        description: String::from(r"
            You can activate this belt as a standard action.
            When you do, you regain $dr3 hit points and increase your \glossterm<fatigue level> by one.
        "),
        upgrades: vec![
            ItemUpgrade::new(4, "Exert to heal $dr5 hit points", "
                The healing increases to $dr5.
            "),
            ItemUpgrade::new(6, "Exert to heal $dr7 hit points", "
                The healing increases to $dr7.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Enlarging Belt"),
        rank: 4,
        short_description: String::from("Increases your size"),
        description: String::from(r"
            You can activate this belt as a standard action.
            When you do, your size increases by one \glossterm<size category>, to a maximum of Huge.
            This effect lasts until you activate the belt again, which returns you to your original size.

            Increasing your size gives you a +1 bonus to Strength for the purpose of determining your \glossterm<weight limits>, a -1 penalty to your Reflex defense, and a -5 penalty to Stealth.
            It also increases your \glossterm<base speed> (see \pcref<Size Categories>).
            This item makes you slightly clumsy in your new size.
            You take a -10 foot penalty to your speed with all of your \glossterm{movement modes}.
        "),
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Shrinking Belt"),
        rank: 3,
        short_description: String::from("Reduces your size"),
        description: String::from(r"
            You can activate this belt as a standard action.
            When you do, your size decreases by one \glossterm<size category>, to a minimum of Tiny.
            This effect lasts until you activate the belt again, which returns you to your original size.

            Reducing your size gives you a -1 penalty to Strength for the purpose of determining your \glossterm<weight limits>, a +1 bonus to your Reflex defense, and a +5 bonus to Stealth.
            It also reduces your \glossterm<base speed> (see \pcref<Size Categories>).
        "),
        upgrades: vec![
            ItemUpgrade::new(7, "Greatly reduces your size", "
                You can choose to decrease your size by two size categories instead of one.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem {
        name: String::from("Utility Belt"),
        rank: 2,
        short_description: String::from("Contains five large pockets"),
        description: String::from(r"
            This belt contains five pockets, each of which is larger on the inside than the outside.
            The inside of each pocket is a six inch cube.
            You can put anything you want in each pocket, but you still carry the weight of anything in the pockets.
            If you put reactive objects in a pocket, such as acid or burning alchemist's fire, it may destroy the pocket until the belt is repaired.

            As long as each pocket is no more than half full, or is full of completely interchangeable items, you can reach into any pocket just as easily as you can reach into a nonmagical pocket.
            Overstuffed pockets may take more time to sift through to find the specific item you want, just like rummaging through a backpack.

            If you take off this belt or stop attuning to it, the items in the belt become inaccessible.
            If this belt is destroyed, the items within it become lost in the Astral Plane.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Contains ten large pockets", "
                The belt has ten pockets instead of five, and each pocket is a one foot cube instead of a six inch cube.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Belt(StandardItem::attribute_item(
        "Belt of Epic Constitution",
        &Attribute::Constitution,
    )));

    apparel
}

// Aerial effects
// Enraging effects (bullfighter)
// Sheltering protection effects
// Stealth effects
fn cloaks() -> Vec<Apparel> {
    let mut apparel = vec![];

    apparel.push(Cloak(StandardItem {
        name: String::from("Cloak of the Noble Rider"),
        rank: 2,
        short_description: String::from("Can exert to save your mount from death"),
        description: String::from(r"
            Whenever a non-humanoid mount that you are riding would gain one or more \glossterm<vital wounds>, this cloak automatically activates.
            When it does, you increase your \glossterm<fatigue level> by one, and the mount does not make a \glossterm<vital roll> for the vital wounds.
            Instead, each vital wound is treated as having a vital roll of 1, which prevents the mount from dying from its wounds.
        "),
        upgrades: vec![
            ItemUpgrade::new(4, "Can save your mount from death", "
                This cloak does not increase your fatigue level when it activates.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Ghost Shroud"),
        rank: 2,
        short_description: String::from("Grants limited ability to hit incorporeal creatures"),
        description: String::from(r"
            Creatures that are \trait<incorporeal> are only \impervious to physical damage you inflict instead of being immune to it.
            In addition, you can touch incorporeal creatures, though they are impervious to any attacks you make to touch them.
        "),
        upgrades: vec![
            ItemUpgrade::new(4, "Grants ability to hit incorporeal creatures", "
                Incorporeal creatures are not impervious to your physical damage and touch attacks.
            "),
            ItemUpgrade::new(7, "Grants ability to easily hit incorporeal creatures", r"
                Incorporeal creatures are \vulnerable to your physical damage and touch attacks.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Cloak of the Unseen Hunter"),
        rank: 5,
        short_description: String::from("Grants +1 accuracy while concealed"),
        description: String::from(r"
            You gain a +1 \glossterm<accuracy> bonus against creatures and objects that you have \glossterm<concealment> from.
        "),
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Cloak of Elemental Endurance"),
        rank: 2,
        short_description: String::from("Grants tolerance of temperature extremes"),
        description: String::from(r"
            You can exist comfortably in conditions between -50 and 140 degrees Fahrenheit without any ill effects.
            You suffer the normal penalties in temperatures outside of that range.
        "),
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Cloak of Death's Door"),
        rank: 2,
        short_description: String::from("Grants +1 Armor at low health"),
        description: String::from(r"
            While you are at or below half your maximum \glossterm<hit points>, you gain a +1 bonus to your Armor defense.
        "),
        upgrades: vec![
            ItemUpgrade::new(4, "Grants +1 defenses at low health", "
                The bonus applies to all defenses.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Cloak of Nondetection"),
        rank: 2,
        short_description: String::from("Impervious to magical detection"),
        description: String::from(r"
            You are \impervious to attacks with the \abilitytag<Detection> or \abilitytag<Scrying> tags.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Immune to magical detection", "
                You are immune instead of impervious.
                This does not help against abilities that do not make attacks, so you can still be seen in scrying sensors that are not targeted on you personally.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Quilled Cloak"),
        rank: 2,
        short_description: String::from("Deals $dr3 damage when grappled"),
        description: String::from(r"
            Whenever a creature grapples you, you immediately deal it $dr3 piercing damage.
            This does not affect creatures that you initiate a grapple with.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Deals $dr6 damage when grappled", "
                The damage increases to $dr6.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Avian Cloak"),
        rank: 3,
        short_description: String::from("Grants a glide speed"),
        description: String::from(r"
            Whenever a creature grapples you, you immediately deal it $dr3 piercing damage.
            This does not affect creatures that you initiate a grapple with.
        "),
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Cloak of Wings"),
        rank: 5,
        short_description: String::from("Grants flight up to 15 feet high"),
        description: String::from(r"
            You gain a \glossterm<fly speed> equal to the \glossterm<base speed> for your size with a maximum height of 15 feet (see \pcref<Flight>).
        "),
        upgrades: vec![
            ItemUpgrade::new(7, "Grants flight up to 30 feet high", "
                The height limit increases to 30 feet.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Assassin's Cloak"),
        rank: 3,
        short_description: String::from("Grants invisibility while inactive"),
        description: String::from(r"
            At the end of each round, if you took no actions that round, you become \trait<invisible> (see \pcref<Invisible>).
            This invisibility ends after you take any action, including \glossterm{movement}.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Grants invisibility while mostly inactive", "
                Making a movement during the movement phase does not prevent you from becoming invisible.
            "),
            ItemUpgrade::new(7, "Grants invisibility while partially inactive", "
                Free actions and minor actions also do not prevent you from becoming invisible.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Vanishing Cloak"),
        rank: 5,
        short_description: String::from("Can teleport silently"),
        description: String::from(r"
            You can activate this cloak as a standard action.
            When you do, you \glossterm<teleport> to an unoccupied location within \rngmed range of your original location.
            As normal for teleportation, you can immediately hide when you reach your destination (see \pcref<Stealth>).
            Unlike most teleportation, this teleportation does not make any noise.
        "),
        upgrades: vec![
            ItemUpgrade::new(5, "Grants invisibility while mostly inactive", "
                Making a movement during the movement phase does not prevent you from becoming invisible.
            "),
            ItemUpgrade::new(7, "Grants invisibility while partially inactive", "
                Free actions and minor actions also do not prevent you from becoming invisible.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Challenger's Cloak"),
        rank: 3,
        short_description: String::from("Increases accuracy penalty on goaded foes by 1"),
        description: String::from(r"
            Each creature suffering penalties for being \goaded by you takes an additional -1 \glossterm<accuracy> penalty against creatures other than you.
        "),
        upgrades: vec![
            ItemUpgrade::new(6, "Increases accuracy penalty on goaded foes by 2", r"
                The penalty increases to -2.
            "),
        ],
        ..Apparel::default()
    }));

    apparel.push(Cloak(StandardItem {
        name: String::from("Cloak of Mist"),
        rank: 4,
        short_description: String::from("Fills nearby area with fog"),
        description: String::from(r"
            At the end of each round, fog \glossterm<briefly> fills a \smallarea radius zone from you.
            This fog does not fully block sight, but it provides \glossterm<concealment>.
            There is no time gap between the disappearance of the old fog and the appearance of the new fog, so you can keep continuous fog cover by staying in the same place or moving slowly.
        "),
        upgrades: vec![
            ItemUpgrade::new(6, "Fills a large area with fog", r"
                The fog's area increases to a \largearea radius.
            "),
        ],
        ..Apparel::default()
    }));

    apparel
}