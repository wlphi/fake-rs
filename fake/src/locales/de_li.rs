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
pub struct LI;

impl Data for LI {
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        // Common Liechtenstein male names (German-influenced)
        "Michael", "Thomas", "Peter", "Andreas", "Martin", "Josef", "Daniel", "Christian",
        "Stefan", "Markus", "David", "Johannes", "Lukas", "Alexander", "Patrick", "Florian",
        "Wolfgang", "Marco", "Matthias", "Roman", "Christoph", "Tobias", "Mario", "Gerhard",
        "Werner", "Hans", "Franz", "Sebastian", "Felix", "Simon", "Anton", "Paul",
        // Common Liechtenstein female names (German-influenced)
        "Maria", "Anna", "Elisabeth", "Julia", "Sarah", "Sandra", "Christine", "Andrea",
        "Katharina", "Eva", "Lisa", "Nicole", "Claudia", "Martina", "Sabine", "Christina",
        "Barbara", "Petra", "Monika", "Melanie", "Karin", "Laura", "Susanne", "Daniela",
        "Verena", "Stefanie", "Birgit", "Magdalena", "Bettina", "Sonja", "Ursula", "Silvia"
    ];
    
    const NAME_LAST_NAME: &'static [&'static str] = &[
        // Common Liechtenstein surnames
        "Marxer", "Frick", "Wenaweser", "Beck", "Büchel", "Frommelt", "Kindle", "Näscher",
        "Ospelt", "Gassner", "Kaiser", "Allgäuer", "Hasler", "Öhri", "Eberle", "Biedermann",
        "Oehri", "Brunhart", "Schädler", "Vogt", "Steiger", "Walser", "Wille", "Burgmeier",
        "Risch", "Lampert", "Kieber", "Elkuch", "Schreiber", "Quaderer", "Hoch", "Meier",
        "Sprenger", "Negele", "Hilti", "Sele", "Batliner", "Götz", "Müller", "Wohlwend",
        "Erne", "Jehle", "Wachter", "Nutt", "Feger", "Kaufmann", "Ritter", "Marogg", "Ott"
    ];
    
    const NAME_TITLE: &'static [&'static str] = &["Frau", "Herr"];

    const ADDRESS_STATE: &'static [&'static str] = &[
        "Vaduz", "Schaan", "Balzers", "Triesen", "Eschen", "Mauren", "Triesenberg", 
        "Ruggell", "Gamprin", "Schellenberg", "Planken"
    ];

    const ADDRESS_STATE_ABBR: &'static [&'static str] = &[
        "VD", "SN", "BZ", "TS", "ES", "MN", "TB", "RG", "GP", "SB", "PK"
    ];

    const ADDRESS_COUNTRY: &'static [&'static str] = &["Liechtenstein", "Fürstentum Liechtenstein"];
    
    const ADDRESS_CITY_SUFFIX: &'static [&'static str] = &[
        "bach", "berg", "dorf", "feld", "hof", "tal", "wald"
    ];

    const ADDRESS_CITY_PREFIX: &'static [&'static str] = &[
        "Ober", "Unter", "Alt", "Neu", "Sankt", "St."
    ];
    
    const ADDRESS_ZIP_FORMATS: &'static [&'static str] = &["####"];
    
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = &[
        "strasse", "gasse", "weg", "platz", "allee", "steig", "ring"
    ];
    
    const ADDRESS_SECONDARY_ADDR_TYPE: &'static [&'static str] =
        &["Haus", "Stock", "Eingang"];

    const ADDRESS_STREET_TPL: &'static str = "{StreetName}{StreetSuffix}";

    const COMPANY_SUFFIX: &'static [&'static str] = &[
        "AG", "GmbH", "Anstalt", "Stiftung", "Foundation", "Establishment", "Trust reg."
    ];
}

impl CityNameGenFn for LI {
    fn gen<R: Rng + ?Sized>(c: &CityName<Self>, rng: &mut R) -> String {
        // Liechtenstein regions and landmarks
        const LOCATIONS: [&str; 6] = [
            "(Rhein)", "(Eschnerberg)"
        ];

        // Common formats for Liechtenstein city names (limited since Liechtenstein has only 11 municipalities)
        const ADDRESS_CITY_WITHOUT_PREFIX: &str = "{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITHOUT_SPACE: &str = "{CityPrefix}{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_SPACE: &str = "{CityPrefix} {CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_LOCATION_TPL: &str = "{CityName} am {Location}";

        let result = match (0..4).fake_with_rng::<u8, _>(rng) {
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
            2 => ADDRESS_CITY_WITH_LOCATION_TPL
                .replace(
                    "{CityName}",
                    Name(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace("{Location}", LOCATIONS[rng.random_range(0..LOCATIONS.len())]),
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
