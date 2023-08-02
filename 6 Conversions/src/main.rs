
use std::convert::TryInto;
use std::convert::TryFrom;
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            println!("even number! from function: {}", value);
            Ok(EvenNumber(value))
          
        } else {
            println!("Not an even number from function: {}", value);
            Err(())
        }
    }

}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    //using match
    let even_number: EvenNumber = match EvenNumber::try_from(8) {
        Ok(dffs) => dffs,
        Err(()) => {
            println!("Not an even number!");
            return;
        }
    };
    println!("Even number: {:?}", even_number);
    //using if let
    if let Ok(even_number) = EvenNumber::try_from(5) {
        println!("Even number: {:?}", even_number);
    } else {
        println!("Not an even number!");
    }

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    //using match
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    match result {
        Ok(even) => println!("Even number: {:?}", even),
        Err(()) => println!("Odd number!"),
    }
    //using if let
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    if let Ok(even) = result {
        println!("Even number: {:?}", even);
    } else {
        println!("Odd number!");
    }
}


