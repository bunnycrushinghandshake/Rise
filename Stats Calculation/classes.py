import util
from util import GOOD, AVERAGE, POOR

class CharacterClass:
    
    def __init__(self, level):
        self.level = level
        self.attack_bonus=dict()
        self.attack_damage=dict()
        self.armor_class=dict()
        for title in util.ac_modifier_titles:
            self.armor_class[title]=dict()
        self.saves=dict()
        for title in util.save_titles:
            self.saves[title]=dict()

        self.set_attack_bonus()
        self.set_attack_damage()
        self.set_armor_class()
        self.set_saves()

    #override with specific classes
    def set_attack_bonus(self):
        pass
    def set_attack_damage(self):
        pass
    def set_armor_class(self):
        pass
    def set_saves(self):
        pass

class Barbarian(CharacterClass):

    base_attack_bonus_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':AVERAGE, 'will':POOR}
    hit_value = 7

    def set_attack_damage(self):
        self.attack_damage['competence']=std_scale(self.level)

    def set_armor_class(self):
        self.armor_class['misc']['inherent']=-2

    def set_saves(self):
        self.saves['fortitude']['competence']=std_scale(self.level)
        self.saves['will']['competence']=std_scale(self.level)

class Bard(CharacterClass):
    base_attack_bonus_progression = AVERAGE
    save_progressions = {'fortitude':AVERAGE, 'reflex':AVERAGE, 'will':AVERAGE}
    hit_value = 6

class Cleric(CharacterClass):
    base_attack_bonus_progression = AVERAGE
    save_progressions = {'fortitude':AVERAGE, 'reflex':POOR, 'will':GOOD}
    hit_value = 5

class Druid(CharacterClass):
    base_attack_bonus_progression = AVERAGE
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':AVERAGE}
    hit_value = 5

class Fighter(CharacterClass):
    base_attack_bonus_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':AVERAGE}
    hit_value = 6

    def set_armor_class(self):
        self.armor_class['dodge']['competence']=(self.level+5)/6

    def set_attack_bonus(self):
        #weapon focus + weapon disciplines
        ab=1+(self.level+3)/6
        if self.level>=8:
            ab+=1
        self.attack_bonus['competence']=ab

class Monk(CharacterClass):
    base_attack_bonus_progression = AVERAGE
    save_progressions = {'fortitude':AVERAGE, 'reflex':GOOD, 'will':GOOD}
    hit_value = 5

class Paladin(CharacterClass):
    base_attack_bonus_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':GOOD}
    hit_value = 6

class Ranger(CharacterClass):
    base_attack_bonus_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':AVERAGE, 'will':AVERAGE}
    hit_value = 6

class Rogue(CharacterClass):
    base_attack_bonus_progression = AVERAGE
    save_progressions = {'fortitude':POOR, 'reflex':GOOD, 'will':POOR}
    hit_value = 5

class Sorcerer(CharacterClass):
    base_attack_bonus_progression = POOR
    save_progressions = {'fortitude':POOR, 'reflex':POOR, 'will':GOOD}
    hit_value = 4

class Wizard(CharacterClass):
    base_attack_bonus_progression = POOR
    save_progressions = {'fortitude':POOR, 'reflex':POOR, 'will':GOOD}
    hit_value = 4

class Warrior(CharacterClass):
    base_attack_bonus_progression = GOOD
    save_progressions = {'fortitude':GOOD, 'reflex':POOR, 'will':POOR}
    hit_value = 6

#+2, +3 at 8th, +4 at 14th, +5 at 20th
def std_scale(level):
    return (level+10)/6
