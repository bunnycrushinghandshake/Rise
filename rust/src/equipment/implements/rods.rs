use crate::equipment::{Implement, ItemUpgrade, StandardItem};
use crate::equipment::Implement::Rod;

pub fn rods() -> Vec<Implement> {
    let mut implements = vec![];

    implements.push(Rod(StandardItem {
        name: String::from("Rod of Flame"),
        rank: 2,
        short_description: String::from(r"Deals $dr1l damage in a cone"),
        description: String::from(r"
            You can activate this rod as a standard action.
            When you do, make an attack vs. Reflex against everything within a \medarea cone.
            Your minimum accuracy is $accuracy.
            \hit $dr1l fire damage.
            \miss Half damage.
        "),
        upgrades: vec![
            ItemUpgrade::new(4, "Deals $dr3l damage in a cone", r"
                The minimum accuracy increases to $accuracy and the damage increases to $dr3l.
            "),
        ],
        ..Implement::default()
    }));

    implements.push(Rod(StandardItem {
        name: String::from("Rod of Dragonflame"),
        rank: 6,
        short_description: String::from(r"Deals $dr5l damage in a large cone"),
        description: String::from(r"
            You can activate this rod as a standard action.
            When you do, make an attack vs. Reflex against everything within a \largearea cone.
            Your minimum accuracy is $accuracy.
            \hit $dr5l fire damage.
            \miss Half damage.
        "),
        ..Implement::default()
    }));

    implements.push(Rod(StandardItem {
        name: String::from("Rod of Translocation"),
        rank: 2,
        short_description: String::from(r"Can teleport up to 30 feet"),
        description: String::from(r"
            You can activate this rod as a standard action.
            When you do, you \glossterm{teleport} to an unoccupied location within \shortrange.
        "),
        upgrades: vec![
            ItemUpgrade::new(4, "Can teleport up to 60 feet", r"
                The range increases to \medrange.
            "),
            ItemUpgrade::new(6, "Can teleport up to 90 feet", r"
                The range increases to \longrange.
            "),
        ],
        ..Implement::default()
    }));

    implements.push(Rod(StandardItem {
        name: String::from("Radiant Rod"),
        rank: 2,
        short_description: String::from("Can deal $dr2l damage"),
        description: String::from(r"
            This rod sheds \glossterm{bright illumination} in a \smallarea radius.
            You can activate it as a standard action.
            When you do, it fires a ray of light at anything within \shortrange.
            Make an attack against the target's Reflex defense.
            Your minimum accuracy is $accuracy.
            Whether you hit or miss, \glossterm{bright illumination} \glossterm{briefly} fills a 30 foot radius around a 5 ft. wide straight line between you and the target.
            \hit $dr2l energy damage.
            If this attack beats a creature's Fortitude defense, it deals maximum damage.
        "),
        upgrades: vec![
            ItemUpgrade::new(4, "Can deal $dr4l damage", r"
                The minimum accuracy increases to $accuracy, and the damage increases to $dr4l.
            "),
            ItemUpgrade::new(6, "Can deal $dr6l damage", r"
                The minimum accuracy increases to $accuracy, and the damage increases to $dr6l.
            "),
        ],
        ..Implement::default()
    }));

    implements
}
