use codebook::{
    parser::{TextRange, WordLocation},
    queries::LanguageType,
};

mod utils;
// WIP PHP Support
#[test]
fn test_php_location() {
    utils::init_logging();
    let sample_text = r#"<?php
// This is a PHP sample file
namespace App\Servicez;

/**
 * A class with some misspellings
 */
class UserServicce {
    // Class constants
    const STATUS_ACTIVVE = 'active';

    // Properties
    private $userIdd;
    protected $databaase;

    // Constructor
    public function __construct($userIdd, $databaase) {
        $this->userIdd = $userIdd;
        $this->databaase = $databaase;
    }

    // Regular method with misspelling
    public function getUserDeetails() {
        $querry = "SELECT * FROM users WHERE id = " . $this->userIdd;

        if (empty($resullt)) {
            throw new \Excepton("User not foundd");
        }

        return $resullt;
    }
}

// Function outside a class
function formattCurrency($amountt, $currency = 'USD') {
    $symboll = '';

    try {
        // Some code that might throw an error
        $formattted = $symboll . number_format($amountt, 2);
    } catch (Excepton $errr) {
        // Handle the error
    }

    return $formattted;
}

// Variable usage
$userr = new UserServicce(123, $dbb);
$userDetails = $userr->getUserDeetails();
?>"#;

    let expected = vec![
        WordLocation::new(
            "Servicez".to_string(),
            vec![TextRange {
                start_char: 14,
                end_char: 22,
                line: 2,
            }],
        ),
        WordLocation::new(
            "Servicce".to_string(),
            vec![
                TextRange {
                    start_char: 10,
                    end_char: 18,
                    line: 7,
                },
                TextRange {
                    start_char: 17,
                    end_char: 25,
                    line: 48,
                },
            ],
        ),
        WordLocation::new(
            "ACTIVVE".to_string(),
            vec![TextRange {
                start_char: 17,
                end_char: 24,
                line: 9,
            }],
        ),
        WordLocation::new(
            "Idd".to_string(),
            vec![
                TextRange {
                    start_char: 17,
                    end_char: 20,
                    line: 12,
                },
                TextRange {
                    start_char: 37,
                    end_char: 40,
                    line: 16,
                },
                TextRange {
                    start_char: 19,
                    end_char: 22,
                    line: 17,
                },
                TextRange {
                    start_char: 30,
                    end_char: 33,
                    line: 17,
                },
                TextRange {
                    start_char: 65,
                    end_char: 68,
                    line: 23,
                },
            ],
        ),
        WordLocation::new(
            "databaase".to_string(),
            vec![
                TextRange {
                    start_char: 15,
                    end_char: 24,
                    line: 13,
                },
                TextRange {
                    start_char: 43,
                    end_char: 52,
                    line: 16,
                },
                TextRange {
                    start_char: 15,
                    end_char: 24,
                    line: 18,
                },
                TextRange {
                    start_char: 28,
                    end_char: 37,
                    line: 18,
                },
            ],
        ),
        WordLocation::new(
            "Deetails".to_string(),
            vec![
                TextRange {
                    start_char: 27,
                    end_char: 35,
                    line: 22,
                },
                TextRange {
                    start_char: 30,
                    end_char: 38,
                    line: 49,
                },
            ],
        ),
        WordLocation::new(
            "querry".to_string(),
            vec![TextRange {
                start_char: 9,
                end_char: 15,
                line: 23,
            }],
        ),
        WordLocation::new(
            "foundd".to_string(),
            vec![TextRange {
                start_char: 42,
                end_char: 48,
                line: 26,
            }],
        ),
        WordLocation::new(
            "formatt".to_string(),
            vec![TextRange {
                start_char: 9,
                end_char: 16,
                line: 34,
            }],
        ),
        WordLocation::new(
            "amountt".to_string(),
            vec![
                TextRange {
                    start_char: 26,
                    end_char: 33,
                    line: 34,
                },
                TextRange {
                    start_char: 48,
                    end_char: 55,
                    line: 39,
                },
            ],
        ),
        WordLocation::new(
            "symboll".to_string(),
            vec![
                TextRange {
                    start_char: 5,
                    end_char: 12,
                    line: 35,
                },
                TextRange {
                    start_char: 23,
                    end_char: 30,
                    line: 39,
                },
            ],
        ),
        WordLocation::new(
            "formattted".to_string(),
            vec![
                TextRange {
                    start_char: 9,
                    end_char: 19,
                    line: 39,
                },
                TextRange {
                    start_char: 12,
                    end_char: 22,
                    line: 44,
                },
            ],
        ),
        WordLocation::new(
            "Excepton".to_string(),
            vec![
                TextRange {
                    start_char: 23,
                    end_char: 31,
                    line: 26,
                },
                TextRange {
                    start_char: 13,
                    end_char: 21,
                    line: 40,
                },
            ],
        ),
        WordLocation::new(
            "errr".to_string(),
            vec![TextRange {
                start_char: 23,
                end_char: 27,
                line: 40,
            }],
        ),
        WordLocation::new(
            "userr".to_string(),
            vec![
                TextRange {
                    start_char: 1,
                    end_char: 6,
                    line: 48,
                },
                TextRange {
                    start_char: 16,
                    end_char: 21,
                    line: 49,
                },
            ],
        ),
        WordLocation::new(
            "dbb".to_string(),
            vec![TextRange {
                start_char: 32,
                end_char: 35,
                line: 48,
            }],
        ),
    ];

    let not_expected = [
        "php",
        "namespace",
        "class",
        "function",
        "private",
        "protected",
        "public",
        "const",
        "try",
        "catch",
        "new",
        "return",
        "throw",
        "empty",
    ];

    let processor = utils::get_processor();
    let misspelled = processor
        .spell_check(sample_text, Some(LanguageType::Php), None)
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
