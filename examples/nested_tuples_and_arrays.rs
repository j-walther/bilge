#![cfg_attr(feature = "nightly", feature(const_convert, const_trait_impl, const_mut_refs, const_maybe_uninit_write))]
#![allow(clippy::unusual_byte_groupings)]
// you can use the "Expand glob import" command on
// use bilge::prelude::*;
// but still need to add Bitsized, Number yourself
use bilge::prelude::{bitsize, u1, u18, u2, u39, Bitsized, DebugBits, DefaultBits, FromBits, Integer, TryFromBits};

// This file basically just informs you that yes, combinations of different nestings work.
// also see `tests/struct.rs`

#[bitsize(39)]
#[derive(FromBits, DebugBits, PartialEq)]
struct Mess {
    field1: (u1, (u2, u8), u1),
    array: [[InnerTupleStruct; 2]; 2],
    fff: u1,
    // big_fumble: [[(InnerTupleStruct, u2); 2]; 2], //12+8+1+16=37
    big_fumble: [[([[(InnerTupleStruct, u2); 2]; 1], u1); 2]; 1],
}

#[bitsize(18)]
#[derive(TryFromBits, DebugBits, PartialEq, DefaultBits)]
struct UnfilledEnumMess {
    big_fumble: [[([[(HaveFun, u2); 2]; 1], u1); 2]; 1],
}

#[bitsize(2)]
#[derive(TryFromBits, Debug, PartialEq, Clone, Copy, Default)]
enum HaveFun {
    No,
    #[default]
    Yes,
    Maybe,
}

// Currently array elements need to be Copy (Clone is not const and we don't have From<&T>).
#[bitsize(2)]
#[derive(Clone, Copy, FromBits, DebugBits, PartialEq)]
struct InnerTupleStruct(u1, bool);

fn main() {
    let field1 = (u1::new(0), (u2::new(0b00), 0b1111_1111), u1::new(1));
    let array = [
        [InnerTupleStruct::from(u2::new(3)), InnerTupleStruct::from(u2::new(0b10))],
        [InnerTupleStruct::from(u2::new(3)), InnerTupleStruct::from(u2::new(0))],
    ];
    let big_fumble = [[
        (
            [[
                (InnerTupleStruct::from(u2::new(3)), u2::new(3)),
                (InnerTupleStruct::from(u2::new(3)), u2::new(3)),
            ]],
            u1::new(0),
        ),
        (
            [[
                (InnerTupleStruct::from(u2::new(0b10)), u2::new(3)),
                (InnerTupleStruct::from(u2::new(3)), u2::new(3)),
            ]],
            u1::new(0),
        ),
    ]];
    let mut mess = Mess::new(field1, array, u1::new(1), big_fumble);
    let mess2 = Mess::from(u39::new(0b0_1_1111_110_0_1_1111_111__1_0011_1011__1__111_1111_1000));
    // dbg!(&mess);
    assert_eq!(mess, mess2);
    assert_eq!(field1, mess.field1());
    assert_eq!(array, mess.array());
    assert_eq!(big_fumble, mess.big_fumble());

    let field1 = (u1::new(0), (u2::new(0b10), 0b1010_0100), u1::new(0));
    mess.set_field1(field1);

    let elem_0 = [InnerTupleStruct::from(u2::new(0)), InnerTupleStruct::from(u2::new(0b01))];
    let elem_1 = [InnerTupleStruct::from(u2::new(0)), InnerTupleStruct::from(u2::new(3))];
    let array = [elem_0, elem_1];
    mess.set_array(array);

    let big_fumble = [[
        (
            [[
                (InnerTupleStruct::from(u2::new(0)), u2::new(0)),
                (InnerTupleStruct::from(u2::new(0b01)), u2::new(0)),
            ]],
            u1::new(0),
        ),
        (
            [[
                (InnerTupleStruct::from(u2::new(0)), u2::new(1)),
                (InnerTupleStruct::from(u2::new(3)), u2::new(0)),
            ]],
            u1::new(0),
        ),
    ]];
    mess.set_big_fumble(big_fumble);

    // dbg!(&mess);
    assert_eq!(field1, mess.field1());
    assert_eq!(array, mess.array());
    assert_eq!(big_fumble, mess.big_fumble());
    // println!("field1: {:?}", tester.field1());
    // println!("array: {:?}", tester.array());
    // println!("big_fumble: {:?}", tester.big_fumble());

    let uem1 = UnfilledEnumMess::try_from(u18::new(0b1_0101_1110_0_1010_1010)).unwrap();
    let uem2 = UnfilledEnumMess::new([[
        ([[(HaveFun::Maybe, u2::new(2)), (HaveFun::Maybe, u2::new(2))]], u1::new(0)),
        ([[(HaveFun::Maybe, u2::new(3)), (HaveFun::Yes, u2::new(1))]], u1::new(1)),
    ]]);
    assert_eq!(uem1.value, uem2.value);
    assert_eq!(uem1, uem2);
    let err = UnfilledEnumMess::try_from(u18::new(0b1_0101_11___11____0_1010_1010));
    assert!(err.is_err());

    // mess.array_at(2); //panics, like it should

    assert_eq!(elem_0, mess.array_at(0));
    assert_eq!(elem_1, mess.array_at(1));
    mess.set_array_at(0, elem_1);
    mess.set_array_at(1, elem_0);
    assert_eq!(elem_1, mess.array_at(0));
    assert_eq!(elem_0, mess.array_at(1));

    let default = UnfilledEnumMess::default();
    println!("{default:?}");
}
