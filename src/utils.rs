use serde::Serialize;
use std::io::Error;

#[derive(Serialize, Debug, PartialEq)]
pub struct Marker {
    lat: i32,
    lng: i32,
}

pub fn read_stdin(input: String) -> Result<Vec<Marker>, Error> {
    let mut markers: Vec<Marker> = Vec::new();

    let binding = input.replace("\n", "");
    let splitted_input: Vec<&str> = binding.split(',').collect();
    let mark = Marker {
        lat: splitted_input[0].parse::<i32>().unwrap(),
        lng: splitted_input[1].parse::<i32>().unwrap(),
    };
    markers.push(mark);

    Ok(markers)
}


#[test]
fn it_works() {
    let actual = read_stdin("1,2".to_string()).unwrap();

    let marker = Marker {
        lat: 1,
        lng: 2
    };
    let expected = vec![marker];
    assert_eq!(actual, expected);
}
