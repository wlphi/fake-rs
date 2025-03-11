use rand::Rng;

use crate::{
    faker::{
        address::raw::{CityName, CityPrefix, CitySuffix},
        impls::address::CityNameGenFn,
        name::raw::{LastName, Name},
    },
    locales::Data,
    Fake,
};
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct CH;

impl Data for CH {
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        // Common Swiss male names
        "Michael", "Thomas", "Daniel", "Christian", "Peter", "David", "Andreas", "Markus", 
        "Stefan", "Martin", "Hans", "Beat", "Marcel", "Patrick", "Simon", "Urs", "Marco", 
        "Reto", "Bruno", "René", "Adrian", "Philipp", "Matthias", "Roger", "Kurt", "Walter",
        "Pascal", "Christoph", "Lukas", "Samuel", "Felix", "Marc", "Jürg", "Roland", "Oliver",
        // Common Swiss female names
        "Maria", "Anna", "Sandra", "Claudia", "Ruth", "Barbara", "Nicole", "Ursula", "Monika",
        "Silvia", "Andrea", "Christine", "Elisabeth", "Petra", "Karin", "Susanne", "Daniela",
        "Sarah", "Esther", "Bettina", "Sabine", "Laura", "Julia", "Franziska", "Manuela",
        "Lisa", "Martina", "Brigitte", "Heidi", "Verena", "Doris", "Christina", "Marianne",
        "Jasmin", "Kathrin", "Corinne", "Rebecca", "Stefanie", "Michelle", "Margrit"
    ];
    
    const NAME_LAST_NAME: &'static [&'static str] = &[
        // Common Swiss surnames
        "Müller", "Schmid", "Keller", "Weber", "Huber", "Meyer", "Schneider", "Steiner", 
        "Fischer", "Baumann", "Brunner", "Gerber", "Widmer", "Baumgartner", "Zimmermann",
        "Moser", "Meier", "Wyss", "Schmid", "Suter", "Bucher", "Roth", "Graf", "Hofmann",
        "Frei", "Lüthi", "Koch", "Marti", "Zbinden", "Egger", "Hofer", "Berger", "Egli",
        "Hess", "Christen", "Schwarz", "Frey", "Arnold", "Vogel", "Kaufmann", "Peter",
        "Bachmann", "Bühler", "Wagner", "Blaser", "Studer", "Kohler", "Zürcher", "Siegenthaler",
        "Bosshard", "Schweizer", "Bühler", "Tanner", "Pfister", "Flückiger", "Ammann",
        "Kälin", "Stalder", "Bieri", "Stucki"
    ];
    
    const NAME_TITLE: &'static [&'static str] = &["Frau", "Herr"];

    const ADDRESS_STATE: &'static [&'static str] = &[
        "Zürich", "Bern", "Luzern", "Uri", "Schwyz", "Obwalden", "Nidwalden", "Glarus",
        "Zug", "Freiburg", "Solothurn", "Basel-Stadt", "Basel-Landschaft", "Schaffhausen",
        "Appenzell Ausserrhoden", "Appenzell Innerrhoden", "St. Gallen", "Graubünden", 
        "Aargau", "Thurgau", "Tessin", "Waadt", "Wallis", "Neuenburg", "Genf", "Jura"
    ];

    const ADDRESS_STATE_ABBR: &'static [&'static str] = &[
        "ZH", "BE", "LU", "UR", "SZ", "OW", "NW", "GL", "ZG", "FR", "SO", "BS", "BL", "SH",
        "AR", "AI", "SG", "GR", "AG", "TG", "TI", "VD", "VS", "NE", "GE", "JU"
    ];

    const ADDRESS_COUNTRY: &'static [&'static str] = &["Schweiz", "Switzerland", "Suisse", "Svizzera"];
    
    const ADDRESS_CITY_SUFFIX: &'static [&'static str] = &[
        "berg", "burg", "wil", "dorf", "bach", "tal", "ingen", "ikon", "wil", "ach", "kon"
    ];

    const ADDRESS_CITY_PREFIX: &'static [&'static str] = &[
        "Ober", "Unter", "Alt", "Neu", "Gross", "Klein", "Sankt", "St."
    ];
    
    const ADDRESS_ZIP_FORMATS: &'static [&'static str] = &["####"];
    
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = &[
        "strasse", "weg", "gasse", "platz", "allee", "steig", "halde", "rain", "matt"
    ];
    
    const ADDRESS_SECONDARY_ADDR_TYPE: &'static [&'static str] =
        &["Gebäude", "Stock", "Etage", "Eingang"];

    const ADDRESS_STREET_TPL: &'static str = "{StreetName}{StreetSuffix}";

    const COMPANY_SUFFIX: &'static [&'static str] = &[
        "AG", "GmbH", "Sarl", "SA", "& Co.", "AG & Co. KG", "und Partner", "& Söhne", "& Cie"
    ];
}

impl CityNameGenFn for CH {
    fn gen<R: Rng + ?Sized>(c: &CityName<Self>, rng: &mut R) -> String {
        // Swiss lakes
        const LAKES: [&str; 8] = [
            "(See)", "(Léman)", "(Vierwaldstättersee)", "(Zürichsee)", "(Thunersee)", 
            "(Brienzersee)", "(Bodensee)", "(Walensee)"
        ];

        // Common formats for Swiss city names
        const ADDRESS_CITY_WITHOUT_PREFIX: &str = "{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITHOUT_SPACE: &str = "{CityPrefix}{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_SPACE: &str = "{CityPrefix} {CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_DASH_TPL: &str = "{CityPrefix}-{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_LAKE_TPL: &str = "{CityName} am {Lake}";

        let result = match (0..5).fake_with_rng::<u8, _>(rng) {
            0 => ADDRESS_CITY_WITHOUT_SPACE
                .replace(
                    "{CityPrefix}",
                    CityPrefix(c.0).fake_with_rng::<&str, _>(rng),
                )
                .replace(
                    "{CityName}",
                    (LastName(c.0).fake_with_rng::<String, _>(rng))
                        .to_lowercase()
                        .as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
            1 => ADDRESS_CITY_WITH_SPACE
                .replace(
                    "{CityPrefix}",
                    CityPrefix(c.0).fake_with_rng::<&str, _>(rng),
                )
                .replace(
                    "{CityName}",
                    LastName(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
            2 => ADDRESS_CITY_WITH_DASH_TPL
                .replace(
                    "{CityPrefix}",
                    CityPrefix(c.0).fake_with_rng::<&str, _>(rng),
                )
                .replace(
                    "{CityName}",
                    Name(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
            3 => ADDRESS_CITY_WITH_LAKE_TPL
                .replace(
                    "{CityName}",
                    Name(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace("{Lake}", LAKES[rng.random_range(0..LAKES.len())]),
            _ => ADDRESS_CITY_WITHOUT_PREFIX
                .replace(
                    "{CityName}",
                    Name(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
        };
        result
    }
}
