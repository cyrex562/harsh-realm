#[derive(Default, Debug, Clone)]
pub struct DieRollResult {
    pub num_dice: u32,
    pub die_size: u32,
    pub modifier: u32,
    pub mod_sign: char,
    pub rolls: Vec<u32>,
    pub total: u32,
}

impl DieRollResult {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.num_dice.to_string());
        result.push_str("d");
        result.push_str(&self.die_size.to_string());
        result.push_str(&self.mod_sign.to_string());
        result.push_str(&self.modifier.to_string());
        result.push_str(" = ");
        result.push_str(&self.total.to_string());
        result.push_str(" (");
        for i in 0..self.rolls.len() {
            result.push_str(&self.rolls[i].to_string());
            if i < self.rolls.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push_str(")");
        result
    }
}

pub fn roll_dice(num_dice: u32, die_size: u32, modifier: u32, mod_sign: char) -> DieRollResult {
    let mut result = DieRollResult::default();
    result.num_dice = num_dice;
    result.die_size = die_size;
    result.modifier = modifier;

    for _i in 0..num_dice {
        let roll = rand::random::<u32>() % die_size + 1;
        result.total += roll;
        result.rolls.push(roll);
    }

    result.mod_sign = mod_sign;
    if mod_sign == '-' {
        result.total -= modifier;
    } else if mod_sign == '*' {
        result.total *= modifier;
    } else if mod_sign == '/' {
        result.total /= modifier;
    } else {
        result.total += modifier;
    }
    result
}
