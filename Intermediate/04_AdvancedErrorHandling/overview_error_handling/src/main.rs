use anyhow::Context;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, format, write};
use std::io;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{msg}")]
struct ParsePaymentInfoError {
    source: Option<anyhow::Error>,
    msg: String,
}

// `from` trait is used to convert value of one type to value of anothe type.
// impl From<ParseIntError> for ParsePaymentInfoError {
//     fn from(e: ParseIntError) -> Self {
//         ParsePaymentInfoError {
//             source: Some(Box::new(e)),
//             msg: None,
//         }
//     }
// }

// impl std::fmt::Debug for ParsePaymentInfoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self} \n\t{}", self.msg)?;

//         // as_ref() is used to give an reference to the box error rather than taking an ownerships of this.
//         if let Some(e) = self.source.as_ref() {
//             write!(f, "\n\nCaused by: \n\t{e:?}")?;
//         }

//         Ok(())
//     }
// }

// impl Display for ParsePaymentInfoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Parsing payment error: invalid payment info")
//     }
// }

// impl Error for ParsePaymentInfoError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.source.as_deref()
//     }
// }

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| {
            s.parse()
                .with_context(|| format!("{s:?} could not be parsed as u32"))
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentInfoError {
            source: Some(e),
            msg: format!("Failed to parse input as numbers. Input: {card}"),
        })?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError {
            source: None,
            msg: format!(
                "Incorrect number of elements. Expected {expected_len} but get {len}. Elements: {numbers:?}"
            ),
        });
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv,
    })
}

#[derive(Error, Debug)]
enum CreditCardError {
    #[error("{0}")]
    InvalidInput(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// impl std::fmt::Debug for CreditCardError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
//             Self::Other(e, msg) => write!(f, "{self}\n\nCaused by: \n\t{e:?}"),
//         }
//     }
// }

// impl Display for CreditCardError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Credit card error: Could not retrieve credit card.")
//     }
// }

// impl Error for CreditCardError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             CreditCardError::InvalidInput(_) => None,
//             CreditCardError::Other(e, _) => Some(e.as_ref()),
//         }
//     }
// }

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards
        .get(name)
        .ok_or(CreditCardError::InvalidInput(format!(
            "No credit card was found for {name}"
        )))?;

    let card = parse_card(card_string)
        .with_context(|| format!("{name}'s could not be parsed."))
        .map_err(|e| CreditCardError::Other(e))?;
    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 05 12 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredid Card Info: {card:?}"),
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_) => println!("\nSomething went wrong! Try again"),
            }

            log::error!("\n{err:?}");
        }
    }
}
