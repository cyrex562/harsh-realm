use crate::dice::roll_dice;

#[derive(Default, Debug, Clone)]
pub struct CharacterStats {
    pub strength: u32,
    pub strength_dm: i32,
    pub dexterity: u32,
    pub dexterity_dm: i32,
    pub endurance: u32,
    pub endurance_dm: i32,
    pub intelligence: u32,
    pub intelligence_dm: i32,
    pub education: u32,
    pub education_dm: i32,
    pub iq: u32,
    pub social_standing: u32,
    pub social_standing_dm: i32,
    pub height: u32,
    pub mass: u32,
    pub skin_tone: String,
    pub hair_color: String,
    pub eye_color: String,
    pub soc_class: String,
    pub soc_desc_title: String,
}

pub fn gen_character_stats() -> CharacterStats {
    let mut out = CharacterStats::default();
    out.strength = roll_dice(2, 6, 0, '+').total;
    out.strength_dm = calc_stat_dm(out.strength);
    out.dexterity = roll_dice(2, 6, 0, '+').total;
    out.dexterity_dm = calc_stat_dm(out.dexterity);
    out.endurance = roll_dice(2, 6, 0, '+').total;
    out.endurance_dm = calc_stat_dm(out.endurance);
    out.intelligence = roll_dice(2, 6, 0, '+').total;
    out.intelligence_dm = calc_stat_dm(out.intelligence);
    out.iq = (out.intelligence * 10) + 30;
    out.education = roll_dice(2, 6, 0, '+').total;
    out.education_dm = calc_stat_dm(out.education);
    out.social_standing = roll_dice(2, 6, 0, '+').total;
    out.social_standing_dm = calc_stat_dm(out.social_standing);
    out.height = calc_height(out.strength);
    out.mass = calc_mass(out.endurance, out.dexterity);
    out.skin_tone = gen_skin_tone();
    out.hair_color = gen_hair_color();
    out.eye_color = gen_eye_color(&out.hair_color);
    (out.soc_class, out.soc_desc_title) = gen_social_standing(out.social_standing); 
    out
}

pub fn calc_stat_dm(stat: u32) -> i32 {
    if stat == 0 {
        return -3;
    } else if stat >= 1 && stat <= 2 {
        return -2;
    } else if stat >= 3 && stat <= 5 {
        return -1;
    } else if stat >= 6 && stat <= 8 {
        return 0;
    } else if stat >= 9 && stat <= 11 {
        return 1;
    } else if stat >= 12 && stat <= 14 {
        return 2;
    } else {
        return 3;
    }
}

pub const BASE_HEIGHT_TABLE: [i32; 16] = [
    0, 140, 145, 150, 155, 160, 165, 170, 175, 180, 185, 190, 195, 200, 205, 210,
];
pub const HEIGHT_MOD_TABLE: [i32; 19] = [
    0, 0, 0, -10, -8, -6, -4, -2, -1, 0, 0, 0, 0, 1, 2, 4, 6, 8, 10,
];

pub fn calc_height(str: u32) -> u32 {
    let mut height = 0;

    // lookup character's strength against array of base heights
    let base_height = BASE_HEIGHT_TABLE[str as usize];
    let height_mod_roll = roll_dice(3, 6, 0, '+').total;
    let height_mod = HEIGHT_MOD_TABLE[height_mod_roll as usize];
    let height = base_height + height_mod;
    height as u32
}

pub const BASE_MASS_TABLE: [u32; 16] = [
    36, 42, 48, 54, 60, 66, 72, 78, 84, 90, 96, 102, 108, 114, 120, 126,
];
pub const MASS_DEX_MOD_TABLE: [i32; 19] = [
    0, 12, 10, 8, 6, 4, 2, 0, 0, 0, -2, -4, -6, -8, -10, -12, 0, 0, 0,
];

pub const MASS_MOD_TABLE: [i32; 19] = [
    0, 0, 0, -12, -10, -8, -6, -4, -2, -1, 0, 0, 1, 2, 4, 6, 8, 10, 12,
];

pub fn calc_mass(end: u32, dex: u32) -> u32 {
    let base_mass = BASE_MASS_TABLE[end as usize];
    let mass_dex_mod = MASS_DEX_MOD_TABLE[dex as usize];
    let mass_mod_roll = roll_dice(3, 6, 3, '-').total;
    let mass_mod = MASS_MOD_TABLE[mass_mod_roll as usize];
    (base_mass as i32 + mass_dex_mod + mass_mod as i32) as u32
}

pub const SKIN_TONE_TABLE: [&'static str; 7] = [
    "",
    "pale white",
    "white",
    "tanned",
    "olive",
    "brown",
    "dark brown",
];

pub const EYE_COLOR_TABLE: [&'static str;5] = [
    "brown",
    "blue",
    "green",
    "hazel",
    "gray"
];

pub fn gen_skin_tone() -> String {
    let skin_tone_roll = roll_dice(1, 6, 0, '+').total;
    SKIN_TONE_TABLE[skin_tone_roll as usize].to_string()
}

pub fn gen_hair_color() -> String {
    let hair_color_roll = roll_dice(2, 6, 0, '+').total;
    if hair_color_roll >= 2 && hair_color_roll <= 5 {
        return "black".to_string();
    } else if hair_color_roll >= 6 && hair_color_roll <= 8 {
        return "brown".to_string();
    } else {
        return "blonde".to_string();
    }
}

pub fn gen_eye_color(hair_color: &str) -> String {
    let eye_color_roll = roll_dice(2, 6, 0, '+').total;

    if eye_color_roll >= 2 && eye_color_roll <= 4 {
        if hair_color == "black" {
            return "brown".to_string();
        } else if hair_color == "brown" {
            return "brown".to_string();
        } else {
            return "blue".to_string();
        }
    } else if eye_color_roll >= 5 && eye_color_roll <=6 {
        if hair_color == "black" {
            return "blue".to_string();
        } else if hair_color == "brown" {
            return "green".to_string();
        } else {
            return "green".to_string();
        }
    } else if eye_color_roll >= 7 && eye_color_roll <= 8 {
        if hair_color == "black" {
            return "green".to_string();
        } else if hair_color == "brown" {
            return "blue".to_string();
        } else {
            return "hazel".to_string();
        }
    } else if eye_color_roll >= 9 && eye_color_roll <= 10 {
        if hair_color == "black" {
            return "hazel".to_string();
        } else if hair_color == "brown" {
            return "hazel".to_string();
        } else {
            return "brown".to_string();
        }
    } else { //if eye_color_roll >= 11 && eye_color_roll <= 12 {
        let gray_roll = roll_dice(2, 6, 0,'+').total;
        if (2..9).contains(&gray_roll) {
            // brown, blue, green, hazel, gray
            let left_roll = roll_dice(1, 5, 0, '+').total;
            let left_color = EYE_COLOR_TABLE[left_roll as usize];
            let right_roll = roll_dice(1, 5, 0, '+').total;
            let right_color = EYE_COLOR_TABLE[right_roll as usize];
            return format!("{}/{}", left_color, right_color);
        }
        else if (10..11).contains(&gray_roll) {
            return "red".to_string();
        }
        else if (12..12).contains(&gray_roll) {
            return "violet".to_string();
        }
        else {
            return "gray".to_string();
        }
    }
}


pub fn gen_social_standing(soc: u32) -> (String,String)
{
    if soc == 0
    {
        return ("No social class".to_string(), "Fugitive wanteed for exceptionally repugnant crimes; no contact with civilization".to_string());
    } else if soc == 1 {
        return ("Lower class".to_string(), "Exile, outcast".to_string());
    } else if soc == 2 {
        return ("Lower class".to_string(), "Very Poor".to_string());
    } else if soc == 3 {
        return ("Lower class".to_string(), "Very low status; deprived background".to_string());
    } else if soc == 4 {
        return ("Lower class".to_string(), "Unskilled laborer;poor".to_string());
    } else if soc == 5 {
        return ("Lower class".to_string(), "Woroking or lower class; semi-skilled".to_string());
    } else if soc == 6 {
        return ("Middle Class".to_string(),
                "Skilled worker".to_string());
    } else if soc == 7 {
        return("Middle Class".to_string(), "Middle Class".to_string());
    } else if soc == 8 {
        return("Middle class".to_string(), "Upper edge of middle class".to_string());
    } else if soc == 9 {
        return("Middle class".to_string(), "Educated or respected professional/executive; a distant relative of a noble family".to_string());
    } else if soc == 10 {
        return("Middle class".to_string(), "Untitled or close member of a noble family".to_string());
    } else if soc == 11 {
        return("Nobility".to_string(), "Knight, Knightess, Dame".to_string())
    } else if soc == 12 {
        return("Nobility".to_string(), "Baron, Baronet, Baroness".to_string());
    } else if soc == 13 {
        return("Nobility".to_string(), "Marquis, Marquessa, Marchioness".to_string());
    } else if soc == 14 {
        return("Nobility".to_string(), "Count, Countess".to_string());
    } 
    // else if (15..15).contains(soc) 
    else
    {
        return("Nobility".to_string(), "Duke, Duchess".to_string());
    }
}