use serde::Serialize;
use std::io::Error;

#[derive(Serialize, Debug, PartialEq)]
pub struct Marker {
    lat: String,
    lng: String,
}

pub fn read_stdin(input: String) -> Result<Vec<Marker>, Error> {
    let mut markers: Vec<Marker> = Vec::new();

    let lines = input.lines();
    for line in lines {
        let splitted_input: Vec<&str> = line.split(',').collect();
        println!("{:?}", splitted_input);
        let mark = Marker {
            lat: splitted_input[0].to_string(),
            lng: splitted_input[1].to_string(),
        };
        markers.push(mark);
    }

    Ok(markers)
}


#[test]
fn it_works() {
    let actual = read_stdin("1,2".to_string()).unwrap();

    let marker = Marker {
        lat: "1".to_string(),
        lng: "2".to_string()
    };
    let expected = vec![marker];
    assert_eq!(actual, expected);
}
