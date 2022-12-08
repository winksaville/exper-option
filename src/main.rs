use std::mem::{discriminant, size_of};

use exper_option::{
    check_box_of_val, check_box_of_val_ret_val_box, check_box_of_val_ret_val_option_box,
    check_option_box_of_val, check_option_box_of_val_ret_val_option_box, check_option_ptr_to_val,
    check_option_val, check_ptr_to_val, check_val,
};

fn main() {
    // Set size_ptr based on "target_pointer_width"
    #[cfg(target_pointer_width = "8")]
    let size_ptr = 1;
    #[cfg(target_pointer_width = "16")]
    let size_ptr = 2;
    #[cfg(target_pointer_width = "32")]
    let size_ptr = 4;
    #[cfg(target_pointer_width = "64")]
    let size_ptr = 8;
    #[cfg(target_pointer_width = "128")]
    let size_ptr = 16;

    #[cfg(all(
        not(target_pointer_width = "1"),
        not(target_pointer_width = "16"),
        not(target_pointer_width = "32"),
        not(target_pointer_width = "64"),
        not(target_pointer_width = "128"),
    ))]
    let size_ptr = 0;
    assert!(size_ptr != 0);

    assert_eq!(size_of::<u8>(), 1);
    println!("size_of::<u8>={}", size_of::<u8>());
    assert_eq!(size_of::<Option<u8>>(), 2);
    println!("size_of::<Option<u8>>={}", size_of::<Option<u8>>());
    assert_eq!(size_of::<u16>(), 2);
    println!("size_of::<u16>={}", size_of::<u16>());
    assert_eq!(size_of::<Option<u16>>(), 4);
    println!("size_of::<Option<u16>>={}", size_of::<Option<u16>>());
    assert_eq!(size_of::<u32>(), 4);
    println!("size_of::<u32>={}", size_of::<u32>());
    assert_eq!(size_of::<Option<u32>>(), 8);
    println!("size_of::<Option<u32>>={}", size_of::<Option<u32>>());
    assert_eq!(size_of::<u64>(), 8);
    println!("size_of::<u64>={}", size_of::<u64>());
    assert_eq!(size_of::<Option<u64>>(), 16);
    println!("size_of::<Option<u64>>={}", size_of::<Option<u64>>());
    assert_eq!(size_of::<u128>(), 16);
    println!("size_of::<u128>={}", size_of::<u128>());
    assert_eq!(size_of::<Option<u128>>(), 24);
    println!("size_of::<Option<u128>>={}", size_of::<Option<u128>>());
    assert_eq!(size_of::<usize>(), 8);
    println!("size_of::<usize>={}", size_of::<usize>());
    assert_eq!(size_of::<Option<usize>>(), 16);
    println!("size_of::<Option<usize>>={}", size_of::<Option<usize>>());

    let var_option_u8_none: Option<u8> = None;
    println!(
        "discrimanent(&var_option_u8_none)={:?}",
        discriminant(&var_option_u8_none)
    );
    let var_option_u8_some: Option<u8> = Some(123);
    println!(
        "discrimanent(var_option_u8_some)={:?}",
        discriminant(&var_option_u8_some)
    );

    println!("size_of::<Option<&u8>>={}", size_of::<Option<&u8>>());
    assert_eq!(size_of::<Option<&u8>>(), size_ptr);

    let var_u8 = 47;
    let var_option_ref_u8_none: Option<&u8> = None;
    println!(
        "discrimanent(&var_option_ref_u8_none)={:?}",
        discriminant(&var_option_ref_u8_none)
    );
    let var_option_ref_var_u8_some: Option<&u8> = Some(&var_u8);
    println!(
        "discrimanent(&var_option_ref_var_u8_some)={:?}",
        discriminant(&var_option_ref_var_u8_some)
    );

    struct UnitStruct;
    assert_eq!(size_of::<UnitStruct>(), 0);
    println!("size_of::<UnitStruct>={}", size_of::<UnitStruct>());
    assert_eq!(size_of::<&UnitStruct>(), size_ptr);
    println!("size_of::<&UnitStruct>={}", size_of::<&UnitStruct>());
    assert_eq!(size_of::<Option<&UnitStruct>>(), size_ptr);
    println!(
        "size_of::<Option<&UnitStruct>>={}",
        size_of::<Option<&UnitStruct>>()
    );

    struct StructU8 {
        _f: u8,
    }
    assert_eq!(size_of::<StructU8>(), 1);
    println!("size_of::<StructU8>={}", size_of::<StructU8>());
    assert_eq!(size_of::<&StructU8>(), size_ptr);
    println!("size_of::<&StructU8>={}", size_of::<&StructU8>());
    assert_eq!(size_of::<Option<&StructU8>>(), size_ptr);
    println!(
        "size_of::<Option<&StructU8>>={}",
        size_of::<Option<&StructU8>>()
    );

    let r = check_option_val(Some(3));
    println!("{r}");
    assert_eq!(r, 3);
    let r = check_option_val(Some(255));
    println!("{r}");
    assert_eq!(r, -1);
    let r = check_option_val(None);
    println!("{r}");
    assert_eq!(r, -2);

    let r = check_val(3);
    println!("{r}");
    assert_eq!(r, 3);
    let r = check_val(255);
    println!("{r}");
    assert_eq!(r, -1);

    let mut val = 3u8;
    let r = check_option_ptr_to_val(Some(&val));
    println!("{r}");
    assert_eq!(r, 3);
    val = 255;
    let r = check_option_ptr_to_val(Some(&val));
    println!("{r}");
    assert_eq!(r, -1);
    let r = check_option_ptr_to_val(None);
    println!("{r}");
    assert_eq!(r, -2);

    let mut val = 3u8;
    let r = check_ptr_to_val(&val);
    println!("{r}");
    assert_eq!(r, 3);
    val = 255;
    let r = check_ptr_to_val(&val);
    println!("{r}");
    assert_eq!(r, -1);

    let mut val = Box::new(3);
    let r = check_option_box_of_val(Some(val));
    println!("{r}");
    assert_eq!(r, 3);
    val = Box::new(255);
    let r = check_option_box_of_val(Some(val));
    println!("{r}");
    assert_eq!(r, -1);
    let r = check_option_box_of_val(None);
    println!("{r}");
    assert_eq!(r, -2);

    let mut val = Box::new(3);
    let r = check_box_of_val(val);
    println!("{r}");
    assert_eq!(r, 3);
    val = Box::new(255);
    let r = check_box_of_val(val);
    println!("{r}");
    assert_eq!(r, -1);

    let mut val = Box::new(3);
    let (r, obv) = check_option_box_of_val_ret_val_option_box(Some(val));
    println!("{r} {obv:?}");
    assert_eq!(r, 3);
    assert_eq!(obv, Some(Box::new(3)));
    val = Box::new(255);
    let (r, obv) = check_option_box_of_val_ret_val_option_box(Some(val));
    println!("{r} {obv:?}");
    assert_eq!(r, -1);
    assert_eq!(obv, Some(Box::new(255)));
    let (r, obv) = check_option_box_of_val_ret_val_option_box(None);
    println!("{r} {obv:?}");
    assert_eq!(r, -2);
    assert_eq!(obv, None);

    let mut val = Box::new(3);
    let (r, obv) = check_box_of_val_ret_val_option_box(val);
    println!("{r} {obv:?}");
    assert_eq!(r, 3);
    assert_eq!(obv, Some(Box::new(3)));
    val = Box::new(255);
    let (r, obv) = check_box_of_val_ret_val_option_box(val);
    println!("{r} {obv:?}");
    assert_eq!(r, -1);
    assert_eq!(obv, Some(Box::new(255)));
    val = Box::new(128);
    let (r, obv) = check_box_of_val_ret_val_option_box(val);
    println!("{r} {obv:?}");
    assert_eq!(r, -2);
    assert_eq!(obv, None);

    let mut val = Box::new(3);
    let (r, obv) = check_box_of_val_ret_val_box(val);
    println!("{r} {obv:?}");
    assert_eq!(r, 3);
    assert_eq!(obv, Box::new(3));
    val = Box::new(255);
    let (r, obv) = check_box_of_val_ret_val_box(val);
    println!("{r} {obv:?}");
    assert_eq!(r, -1);
    assert_eq!(obv, Box::new(255));
}
