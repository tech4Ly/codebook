use codebook::{
    parser::{TextRange, WordLocation},
    queries::LanguageType,
};

mod utils;

#[test]
fn test_typescript_location() {
    utils::init_logging();
    let sample_text = r#"
    import { Component } from 'react';

    interface UserProifle {
        id: number;
        firstName: string;
        lastName: string;
        emailAdress: string;
        isActtive: boolean;
    }

    class UserManagger extends Component {
        private userz: UserProifle[] = [];

        constructor(private apiEndpoont: string) {
            super();
        }

        public async fetchUsars(): Promise<UserProifle[]> {
            try {
                const respoonse = await fetch(this.apiEndpoont);
                return await respoonse.json();
            } catch (erorr) {
                console.log("Fetching usars failled:", erorr);
                return [];
            }
        }
    }"#;

    let expected = vec![
        WordLocation::new(
            "Proifle".to_string(),
            vec![TextRange {
                start_char: 18,
                end_char: 25,
                line: 3,
            }],
        ),
        WordLocation::new(
            "Adress".to_string(),
            vec![TextRange {
                start_char: 13,
                end_char: 19,
                line: 7,
            }],
        ),
        WordLocation::new(
            "Acttive".to_string(),
            vec![TextRange {
                start_char: 10,
                end_char: 17,
                line: 8,
            }],
        ),
        WordLocation::new(
            "Managger".to_string(),
            vec![TextRange {
                start_char: 14,
                end_char: 22,
                line: 11,
            }],
        ),
        WordLocation::new(
            "userz".to_string(),
            vec![TextRange {
                start_char: 16,
                end_char: 21,
                line: 12,
            }],
        ),
        WordLocation::new(
            "Endpoont".to_string(),
            vec![TextRange {
                start_char: 31,
                end_char: 39,
                line: 14,
            }],
        ),
        WordLocation::new(
            "Usars".to_string(),
            vec![TextRange {
                start_char: 26,
                end_char: 31,
                line: 18,
            }],
        ),
        WordLocation::new(
            "respoonse".to_string(),
            vec![TextRange {
                start_char: 22,
                end_char: 31,
                line: 20,
            }],
        ),
        WordLocation::new(
            "erorr".to_string(),
            vec![TextRange {
                start_char: 21,
                end_char: 26,
                line: 22,
            }],
        ),
        WordLocation::new(
            "usars".to_string(),
            vec![TextRange {
                start_char: 38,
                end_char: 43,
                line: 23,
            }],
        ),
        WordLocation::new(
            "failled".to_string(),
            vec![TextRange {
                start_char: 44,
                end_char: 51,
                line: 23,
            }],
        ),
    ];

    let not_expected = [
        "import",
        "Component",
        "react",
        "interface",
        "number",
        "string",
        "boolean",
        "class",
        "extends",
        "private",
        "constructor",
        "super",
        "public",
        "async",
        "Promise",
        "try",
        "const",
        "await",
        "fetch",
        "return",
        "json",
        "catch",
        "console",
        "log",
    ];

    let processor = utils::get_processor();
    let misspelled = processor
        .spell_check(sample_text, Some(LanguageType::Typescript), None)
        .to_vec();

    println!("Misspelled words: {misspelled:?}\n");

    for e in &expected {
        println!("Expecting: {e:?}");
        let miss = misspelled
            .iter()
            .find(|r| r.word == e.word)
            .expect("Word not found");
        assert_eq!(miss.locations, e.locations);
    }

    for result in misspelled {
        assert!(!not_expected.contains(&result.word.as_str()));
    }
}
