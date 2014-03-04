import util

def barbarian_dr(level, creature):
    dr_value = level
    creature.damage_reduction = util.DamageReduction(dr_value,
            'physical')

def danger_sense(level, creature):
    creature.initiative.add_competence(level/2)

def larger_than_life(creature):
    creature.weapon_damage.die.increase_size(increase_min=True)

def larger_than_belief(creature):
    creature.weapon_damage.die.increase_size(increase_min=True)
    
def rage(level, creature):
    creature.weapon_damage.add_competence(util.std_scale(level))
    creature.armor_class.misc.add_circumstance(-2)
    creature.saves.fortitude.add_competence(util.std_scale(level))
    creature.saves.will.add_competence(util.std_scale(level))
    creature.current_hit_points += level*util.std_scale(level)

class Feat(object):

    def __init__(self):
        pass

    def meets_prerequisites(self, creature):
        return True

    def apply_benefit(self, creature):
        pass

class OverwhelmingForce(Feat):
    def meets_prerequisites(self, creature):
        return creature.attributes.strength.get_total() >=5 and creature.attack_bonus.base_attack_bonus >=8 and creature.weapon.encumbrance == 'heavy'

    def apply_benefit(self, creature):
        creature.weapon_damage.add_inherent(
                (creature.attributes.strength.get_total()+1)/2)
