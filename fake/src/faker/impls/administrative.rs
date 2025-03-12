use crate::faker::administrative::raw::*;
use crate::locales::{FR_FR, DE_AT, DE_CH, DE_LI, IT_IT, DE_DE};
use crate::{Dummy, Fake};
use rand::seq::IndexedRandom;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use std::iter;

// ref https://fr.wikipedia.org/wiki/Num%C3%A9rotation_des_d%C3%A9partements_fran%C3%A7ais
const FR_FR_DEPARTMENTS: &[&str] = &[
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "2A", "2B", "21", "22", "23", "24", "25", "26", "27", "28", "29", "31", "32", "33",
    "34", "35", "36", "37", "38", "39", "41", "42", "43", "44", "45", "46", "47", "48", "49", "51",
    "52", "53", "54", "55", "56", "57", "58", "59", "61", "62", "63", "64", "65", "66", "67", "68",
    "69", "71", "72", "73", "74", "75", "76", "77", "78", "79", "81", "82", "83", "84", "85", "86",
    "87", "88", "89", "91", "92", "93", "94", "95", "971", "972", "973", "974", "975", "976",
    "977", "978", "984", "986", "987", "988", "989",
];

// ref https://de.wikipedia.org/wiki/Politische_Bezirke_%C3%96sterreichs
const AT_DISTRICTS: &[&str] = &[
    "101", "102", "103", "201", "202", "203", "204", "205", "206", "207", "208", "209", "210",
    "301", "302", "303", "304", "305", "306", "307", "308", "309", "310", "311", "312", "313",
    "314", "315", "316", "317", "318", "319", "320", "321", "322", "323", "401", "402", "403",
    "404", "405", "406", "407", "408", "409", "410", "411", "412", "413", "414", "415", "416",
    "417", "418", "501", "502", "503", "504", "505", "506", "601", "602", "603", "604", "605",
    "606", "607", "608", "701", "702", "703", "704", "705", "706", "707", "708", "709", "801",
    "802", "803", "804", "805", "806", "807", "808", "809", "901", "902", "903", "904", "905",
    "906", "907", "908", "909",
];

// ref https://en.wikipedia.org/wiki/Cantons_of_Switzerland
const CH_CANTONS: &[&str] = &[
    "AG", "AI", "AR", "BE", "BL", "BS", "FR", "GE", "GL", "GR", "JU", "LU", "NE", "NW", "OW",
    "SG", "SH", "SO", "SZ", "TG", "TI", "UR", "VD", "VS", "ZG", "ZH",
];

// Liechtenstein has 11 municipalities
const LI_MUNICIPALITIES: &[&str] = &[
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11",
];

// Italian province codes for Codice Fiscale
// ref https://en.wikipedia.org/wiki/Province_of_Italy
const IT_PROVINCE_CODES: &[&str] = &[
    "AG", "AL", "AN", "AO", "AP", "AQ", "AR", "AT", "AV", "BA", "BG", "BI", "BL", "BN", "BO",
    "BR", "BS", "BT", "BZ", "CA", "CB", "CE", "CH", "CL", "CN", "CO", "CR", "CS", "CT", "CZ",
    "EN", "FC", "FE", "FG", "FI", "FM", "FR", "GE", "GO", "GR", "IM", "IS", "KR", "LC", "LE",
    "LI", "LO", "LT", "LU", "MB", "MC", "ME", "MI", "MN", "MO", "MS", "MT", "NA", "NO", "NU",
    "OR", "PA", "PC", "PD", "PE", "PG", "PI", "PN", "PO", "PR", "PT", "PU", "PV", "PZ", "RA",
    "RC", "RE", "RG", "RI", "RM", "RN", "RO", "SA", "SI", "SO", "SP", "SR", "SS", "SU", "SV",
    "TA", "TE", "TN", "TO", "TP", "TR", "TS", "TV", "UD", "VA", "VB", "VC", "VE", "VI", "VR",
    "VT", "VV",
];

// Months encoding for Italian Codice Fiscale
const IT_MONTHS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'P', 'R', 'S', 'T',
];

impl Dummy<HealthInsuranceCode<FR_FR>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HealthInsuranceCode<FR_FR>, rng: &mut R) -> Self {
        // ref https://www.previssima.fr/lexique/numero-de-securite-sociale-a-13-chiffres.html
        // and test on http://marlot.org/util/calcul-de-la-cle-nir.php
        let sex: u8 = (1..3).fake_with_rng::<u8, _>(rng);
        let birth_year: u8 = (0..99).fake_with_rng::<u8, _>(rng);
        let birth_month: u8 = (1..13).fake_with_rng::<u8, _>(rng);
        let department: &str = FR_FR_DEPARTMENTS.choose(rng).unwrap();
        let town_code: u16 = (0..999).fake_with_rng::<u16, _>(rng);
        let order_code: u16 = (0..999).fake_with_rng::<u16, _>(rng);
        let department_code: u16 = match department {
            "2A" => 19,
            "2B" => 18,
            _ => department.parse::<u16>().unwrap(),
        };
        let number = format!(
            "{}{:02}{:02}{}{:03}{:03}",
            sex, birth_year, birth_month, department_code, town_code, order_code
        )
        .parse::<u64>()
        .unwrap();
        let key = 97 - (number % 97);
        format!(
            "{} {:02} {:02} {} {:03} {:03} {}",
            sex, birth_year, birth_month, department, town_code, order_code, key
        )
    }
}

impl Dummy<HealthInsuranceCode<AT>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HealthInsuranceCode<AT>, rng: &mut R) -> Self {
        // ref https://www.finanz.at/steuern/sozialversicherungsnummer/
        // Austrian social security number format: XXXX DDMMYY
        let country_code = "AT";
        let regional_code = AT_DISTRICTS.choose(rng).unwrap();
        let birth_day: u8 = (1..29).fake_with_rng::<u8, _>(rng);
        let birth_month: u8 = (1..13).fake_with_rng::<u8, _>(rng);
        let birth_year: u8 = (0..99).fake_with_rng::<u8, _>(rng);
        let serial_number: u16 = (1000..10000).fake_with_rng::<u16, _>(rng);
        
        // Calculate check digit for Austrian social security number
        let number = format!("{}{:02}{:02}", serial_number, birth_day, birth_month);
        let digits: Vec<u32> = number
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        
        let weighted_sum: u32 = digits[0] * 3 + digits[1] * 7 + digits[2] * 9 + 
                                digits[3] * 5 + digits[4] * 8 + digits[5] * 4 + 
                                digits[6] * 2 + digits[7] * 1 + digits[8] * 6;
        
        let check_digit = (weighted_sum % 11) % 10;
        
        format!(
            "{}-{}-{:04} {:02}{:02}{:02}-{}",
            country_code, regional_code, serial_number, birth_day, birth_month, birth_year, check_digit
        )
    }
}

impl Dummy<HealthInsuranceCode<CH>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HealthInsuranceCode<CH>, rng: &mut R) -> Self {
        // ref https://www.ahv-iv.ch/en/Leaflets-forms/Leaflets/International/AHV-Insurance-Number
        // Swiss AHV number format: 756.XXXX.XXXX.XX
        let country_prefix = "756"; // Swiss country code for social security
        let canton = CH_CANTONS.choose(rng).unwrap();
        let first_group: u16 = (1000..10000).fake_with_rng::<u16, _>(rng);
        let second_group: u16 = (1000..10000).fake_with_rng::<u16, _>(rng);
        
        // Calculate check digit for Swiss AHV number
        let number = format!("{}{:04}{:04}", country_prefix, first_group, second_group);
        let digits: Vec<u32> = number
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        
        let sum: u32 = digits.iter().enumerate().map(|(i, &d)| {
            if i % 2 == 0 {
                d
            } else {
                d * 3
            }
        }).sum();
        
        let check_digit = (10 - (sum % 10)) % 10;
        
        format!(
            "CH-{}-{}.{:04}.{:04}.{:02}",
            canton, country_prefix, first_group, second_group, check_digit
        )
    }
}

impl Dummy<HealthInsuranceCode<LI>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HealthInsuranceCode<LI>, rng: &mut R) -> Self {
        // Liechtenstein uses a similar format to Switzerland
        // ref https://www.ahv.li/en/
        let country_prefix = "438"; // Liechtenstein country code for social security
        let municipality = LI_MUNICIPALITIES.choose(rng).unwrap();
        let first_group: u16 = (1000..10000).fake_with_rng::<u16, _>(rng);
        let second_group: u16 = (1000..10000).fake_with_rng::<u16, _>(rng);
        
        // Calculate check digit
        let number = format!("{}{:04}{:04}", country_prefix, first_group, second_group);
        let digits: Vec<u32> = number
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        
        let sum: u32 = digits.iter().enumerate().map(|(i, &d)| {
            if i % 2 == 0 {
                d
            } else {
                d * 3
            }
        }).sum();
        
        let check_digit = (10 - (sum % 10)) % 10;
        
        format!(
            "LI-{}-{}.{:04}.{:04}.{:02}",
            municipality, country_prefix, first_group, second_group, check_digit
        )
    }
}

impl Dummy<FiscalCode<IT>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FiscalCode<IT>, rng: &mut R) -> Self {
        // Italian Codice Fiscale
        // Format: CCCNNN00X00X000X
        // ref: https://en.wikipedia.org/wiki/Italian_fiscal_code
        
        // Generate surname part (3 consonants or filled with vowels if not enough consonants)
        let surname_chars: String = rng
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_alphabetic())
            .take(6)
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
            
        let surname_part: String = surname_chars
            .chars()
            .filter(|&c| !"AEIOU".contains(c))
            .take(3)
            .collect();
        
        let surname_part = if surname_part.len() < 3 {
            // If not enough consonants, add vowels
            let vowels: String = surname_chars
                .chars()
                .filter(|&c| "AEIOU".contains(c))
                .collect();
                
            format!("{}{}", surname_part, vowels)
                .chars()
                .take(3)
                .collect()
        } else {
            surname_part
        };
        
        // Generate name part (3 consonants, rules similar to surname)
        let name_chars: String = rng
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_alphabetic())
            .take(6)
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
            
        let name_consonants: Vec<char> = name_chars
            .chars()
            .filter(|&c| !"AEIOU".contains(c))
            .collect();
            
        let name_part = if name_consonants.len() >= 4 {
            // Rule: if 4+ consonants, take 1st, 3rd, 4th
            format!("{}{}{}", name_consonants[0], name_consonants[2], name_consonants[3])
        } else if name_consonants.len() >= 3 {
            // Take first 3 consonants
            name_consonants.iter().take(3).collect::<String>()
        } else {
            // If not enough consonants, add vowels
            let vowels: String = name_chars
                .chars()
                .filter(|&c| "AEIOU".contains(c))
                .collect();
                
            format!("{}{}", name_consonants.iter().collect::<String>(), vowels)
                .chars()
                .take(3)
                .collect()
        };
        
        // Year of birth (last 2 digits)
        let year: u8 = (0..99).fake_with_rng::<u8, _>(rng);
        
        // Month of birth (encoded as letter A-T)
        let month_index = rng.gen_range(0..12);
        let month_code = IT_MONTHS[month_index];
        
        // Day of birth (1-31 for males, 41-71 for females)
        let sex = if rng.gen_bool(0.5) { 'M' } else { 'F' };
        let day = rng.gen_range(1..=28);
        let day_code = if sex == 'F' { day + 40 } else { day };
        
        // Place of birth (province code)
        let place = IT_PROVINCE_CODES.choose(rng).unwrap();
        
        // Generate base code
        let base_code = format!(
            "{}{}{:02}{}{:02}{}",
            surname_part,
            name_part,
            year,
            month_code,
            day_code,
            place
        );
        
        // Calculate check character
        let odd_sum: u32 = base_code
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, c)| {
                match c {
                    '0'|'A' => 1, '1'|'B' => 0, '2'|'C' => 5, '3'|'D' => 7, '4'|'E' => 9,
                    '5'|'F' => 13, '6'|'G' => 15, '7'|'H' => 17, '8'|'I' => 19, '9'|'J' => 21,
                    'K' => 2, 'L' => 4, 'M' => 18, 'N' => 20, 'O' => 11, 'P' => 3, 'Q' => 6,
                    'R' => 8, 'S' => 12, 'T' => 14, 'U' => 16, 'V' => 10, 'W' => 22, 'X' => 25,
                    'Y' => 24, 'Z' => 23, _ => 0,
                }
            })
            .sum();
            
        let even_sum: u32 = base_code
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, c)| {
                match c {
                    '0'|'A' => 0, '1'|'B' => 1, '2'|'C' => 2, '3'|'D' => 3, '4'|'E' => 4,
                    '5'|'F' => 5, '6'|'G' => 6, '7'|'H' => 7, '8'|'I' => 8, '9'|'J' => 9,
                    'K' => 10, 'L' => 11, 'M' => 12, 'N' => 13, 'O' => 14, 'P' => 15, 'Q' => 16,
                    'R' => 17, 'S' => 18, 'T' => 19, 'U' => 20, 'V' => 21, 'W' => 22, 'X' => 23,
                    'Y' => 24, 'Z' => 25, _ => 0,
                }
            })
            .sum();
            
        let remainder = (odd_sum + even_sum) % 26;
        let check_letter = (b'A' + remainder as u8) as char;
        
        format!("{}{}", base_code, check_letter)
    }
}

impl Dummy<TaxIdentificationNumber<DE>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &TaxIdentificationNumber<DE>, rng: &mut R) -> Self {
        // German Steueridentifikationsnummer
        // Format: 11 digits (10 digits + 1 check digit)
        // ref: https://www.gesetze-im-internet.de/ao_1977/__139b.html
        
        // Generate first 10 digits
        let mut digits: Vec<u8> = vec![];
        for _ in 0..10 {
            digits.push(rng.gen_range(0..10));
        }
        
        // Apply German TIN check digit algorithm (11er-Verfahren mit Gewichtung)
        // ref: https://de.wikipedia.org/wiki/Steueridentifikationsnummer
        let mut product_sum = 0;
        let mut digit_sum = 0;
        
        for (i, &digit) in digits.iter().enumerate() {
            if i == 0 {
                product_sum += digit;
            } else {
                product_sum += (digit + product_sum) % 10 + 1;
            }
            digit_sum += digit;
        }
        
        let check_digit = (11 - (product_sum % 11)) % 10;
        digits.push(check_digit);
        
        // Format the TIN with spaces for readability
        let formatted: String = digits
            .iter()
            .map(|&d| d.to_string())
            .collect();
            
        // Format: XX XXX XXX XXX
        format!("{} {} {} {}", 
            &formatted[0..2],
            &formatted[2..5],
            &formatted[5..8],
            &formatted[8..11]
        )
    }
}

impl Dummy<TaxIdentificationNumber<AT>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &TaxIdentificationNumber<AT>, rng: &mut R) -> Self {
        // Austrian tax identification number (FinanzOnline)
        // Format: 9 digits (2-3-4 format)
        // ref: https://www.help.gv.at/Portal.Node/hlpd/public/content/340/Seite.34060802.html
        
        // Generate 9 digits
        let area_code = rng.gen_range(10..100); // 2 digits
        let office_code = rng.gen_range(100..1000); // 3 digits
        let serial = rng.gen_range(1000..10000); // 4 digits
        
        format!("{}-{}-{}", area_code, office_code, serial)
    }
}
