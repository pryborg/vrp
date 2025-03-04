use super::*;
use crate::core::models::common::Profile;

fn get_index() -> CoordIndex {
    let mut index = CoordIndex::default();
    index.collect((0, 0));
    index.collect((2, 1));

    index
}

#[test]
fn can_create_transport_without_rounding() {
    let index = get_index();

    let transport = index.create_transport(false).unwrap();

    assert_eq!(transport.distance(&Profile::new(0, None), 0, 1, 0.), 2.23606797749979);
}

#[test]
fn can_create_transport_with_rounding() {
    let index = get_index();

    let transport = index.create_transport(true).unwrap();

    assert_eq!(transport.distance(&Profile::new(0, None), 0, 1, 0.), 2.);
}
