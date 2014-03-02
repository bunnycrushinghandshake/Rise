import util
from util import GOOD, AVERAGE, POOR

class CharacterClass:
    
    def __init__(self, level):
        self.level = level

    def apply_progressions(self, base_creature):
        base_creature.attack_bonus.set_progression(self.bab_progression)
        base_creature.saves.set_progressions_dict(self.save_progressions)
        base_creature.hit_value = self.hit_value

    #Inherited classes overwrite
    def apply_modifications(self, base_creature):
        pass

class Average(CharacterClass):
        bab_progression = AVERAGE
        save_progressions = {'fortitude': AVERAGE, 'reflex': AVERAGE, 'will': AVERAGE}
        hit_value = 5
    
class Barbarian(CharacterClass):

    bab_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':AVERAGE, 'will':POOR}
    hit_value = 7

    def apply_modifications(self, base_creature):
        base_creature.attack_damage.add_competence(std_scale(self.level))
        base_creature.armor_class.misc.add_inherent(-2)
        base_creature.saves.fortitude.add_competence(std_scale(self.level))
        base_creature.saves.will.add_competence(std_scale(self.level))
        base_creature.damage_reduction = self.level

class Bard(CharacterClass):
    bab_progression = AVERAGE
    save_progressions = {'fortitude':AVERAGE, 'reflex':AVERAGE, 'will':AVERAGE}
    hit_value = 6

class Cleric(CharacterClass):
    bab_progression = AVERAGE
    save_progressions = {'fortitude':AVERAGE, 'reflex':POOR, 'will':GOOD}
    hit_value = 5

class Druid(CharacterClass):
    bab_progression = AVERAGE
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':AVERAGE}
    hit_value = 5

class Fighter(CharacterClass):
    bab_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':AVERAGE}
    hit_value = 6

    def apply_modifications(self, base_creature):
        base_creature.armor_class.dodge.add_competence((self.level+5)/6)

        #weapon focus + weapon disciplines
        ab=1+(self.level+3)/6
        if self.level>=8:
            ab+=1
        base_creature.attack_bonus.add_competence(ab)

class Monk(CharacterClass):
    bab_progression = AVERAGE
    save_progressions = {'fortitude':AVERAGE, 'reflex':GOOD, 'will':GOOD}
    hit_value = 5

class Paladin(CharacterClass):
    bab_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':GOOD}
    hit_value = 6

class Ranger(CharacterClass):
    bab_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':AVERAGE, 'will':AVERAGE}
    hit_value = 6

class Rogue(CharacterClass):
    bab_progression = AVERAGE
    save_progressions = {'fortitude':POOR, 'reflex':GOOD, 'will':POOR}
    hit_value = 5

class Sorcerer(CharacterClass):
    bab_progression = POOR
    save_progressions = {'fortitude':POOR, 'reflex':POOR, 'will':GOOD}
    hit_value = 4

class Wizard(CharacterClass):
    bab_progression = POOR
    save_progressions = {'fortitude':POOR, 'reflex':POOR, 'will':GOOD}
    hit_value = 4

class Warrior(CharacterClass):
    bab_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':POOR}
    hit_value = 6

#+2, +3 at 8th, +4 at 14th, +5 at 20th
def std_scale(level):
    return (level+10)/6
