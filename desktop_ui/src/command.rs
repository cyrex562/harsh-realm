use regex::Regex;

use crate::{
    character_stats::gen_character_stats,
    dice::{roll_dice, DieRollResult},
    TextLine, TextType,
};

pub fn process_string(text_input_value: &str) -> TextLine {
    // strings starting with "/" are commands
    if text_input_value.starts_with("/") {
        // figure out if the remainder of the string is a valid command
        let raw_command = &text_input_value[1..].to_lowercase();
        if raw_command.starts_with("help") {
            // return String::from("help: TODO");
            return TextLine {
                text: String::from(
                    "help:\n\
                /help - this help text\n\
                /roll - roll dice\n\
                /gen_stats - generate character stats\n\
                ",
                ),
                text_type: TextType::Help,
            };
        } else if raw_command.starts_with("roll") {
            match process_roll_command(raw_command) {
                Ok(value) => {
                    return TextLine {
                        text: value,
                        text_type: TextType::CommandOutput,
                    }
                }
                Err(value) => {
                    return TextLine {
                        text: value,
                        text_type: TextType::CommandOutput,
                    }
                }
            };
        } else if raw_command.starts_with("gen_stats") {
            let char_stats = gen_character_stats();
            return TextLine {
                text: format!(
                    "Strength: {} (DM {})\n\
            Dexterity: {} (DM {})\n\
            Endurance: {} (DM {})\n\
            Intelligence: {} (DM {})\n\
            IQ: {}\n\
            Education: {} (DM {})\n\
            Social Standing: {} (DM {})(class: {}, description/title: {})\n\
            Height: {} cm\n\
            Mass: {} kg\n\
            Skin Tone: \"{}\"\n\
            Hair Color: \"{}\"\n\
            Eye Color: \"{}\"",
                    char_stats.strength,
                    char_stats.strength_dm,
                    char_stats.dexterity,
                    char_stats.dexterity_dm,
                    char_stats.endurance,
                    char_stats.endurance_dm,
                    char_stats.intelligence,
                    char_stats.intelligence_dm,
                    char_stats.iq,
                    char_stats.education,
                    char_stats.education_dm,
                    char_stats.social_standing,
                    char_stats.social_standing_dm,
                    char_stats.soc_class,
                    char_stats.soc_desc_title,
                    char_stats.height,
                    char_stats.mass,
                    char_stats.skin_tone,
                    char_stats.hair_color,
                    char_stats.eye_color
                ),
                text_type: TextType::CommandOutput,
            };
        } else {
            return TextLine {
                text: format!("invalid command: \"{}\"", raw_command),
                text_type: TextType::Error,
            };
        }
    } else {
        return TextLine {
            text: format!("unknown statement: \"{}\"", text_input_value),
            text_type: TextType::Error,
        };
    }
}

pub fn process_roll_command(raw_command: &String) -> Result<String, String> {
    let result_struct: DieRollResult;
    let re1 = Regex::new(r"roll\s*d66").unwrap();
    let re2 = Regex::new(r"roll\s*(\d+)\s*d\s*(\d+)\s*([\+-/\*])\s*(\d+)").unwrap();
    let re3 = Regex::new(r"roll\s*(\d+)\s*d\s*(\d+)").unwrap();
    if re1.is_match(raw_command) {
        result_struct = roll_dice(2, 6, 0, '+');
        return Ok(format!(
            "{}{} ({},{})",
            result_struct.rolls[0],
            result_struct.rolls[1],
            result_struct.rolls[0],
            result_struct.rolls[1]
        ));
    } else if re2.is_match(raw_command) {
        let caps = re2.captures(raw_command).unwrap();
        let num_dice_str = &caps[1];
        let num_dice: u32 = num_dice_str.parse().unwrap();
        let die_size_str = &caps[2];
        let die_size: u32 = die_size_str.parse().unwrap();
        let mod_sign_str: &str = &caps[3];
        let mod_val_str = &caps[4];
        let mod_val: u32 = mod_val_str.parse().unwrap();
        result_struct = roll_dice(
            num_dice,
            die_size,
            mod_val,
            mod_sign_str.chars().nth(0).unwrap(),
        );
        return Ok(result_struct.to_string());
    } else if re3.is_match(raw_command) {
        let caps = re3.captures(raw_command).unwrap();
        let num_dice_str = &caps[1];
        let num_dice: u32 = num_dice_str.parse().unwrap();
        let die_size_str = &caps[2];
        let die_size: u32 = die_size_str.parse().unwrap();
        result_struct = roll_dice(num_dice, die_size, 0, '+');
        return Ok(result_struct.to_string());
    } else {
        return Ok(format!("invalid roll dice string: \"{}\"", raw_command));
    }
}
