use super::Translation;

pub fn translate(key: &'static str, translations: &Vec<Translation>) -> String {
    let translation = translations
        .iter()
        .find(|t| t.key == key);

    if let Some(translation) = translation {
        return translation.value.clone();
    }

    key.into()
}

pub fn parse_translations(file: &'static str) -> Vec<Translation> {
    let mut translations = vec![];

    for line in file.lines() {
        if let Some(pos) = line.find(":") {
            let key = &line[..pos];
            let value = &line[pos + 1..];

            translations.push(Translation::from(key, value));
        }
    }

    translations
}