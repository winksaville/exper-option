pub fn check_option_val(param: Option<u8>) -> isize {
    if let Some(val) = param {
        if val < 128 {
            val as isize
        } else {
            -1
        }
    } else {
        -2
    }
}

//#[inline(never)]
pub fn check_val(param: u8) -> isize {
    if param < 128 {
        param as isize
    } else {
        -1
    }
}

pub fn check_option_ptr_to_val(param: Option<&u8>) -> isize {
    if let Some(val) = param {
        if *val < 128 {
            *val as isize
        } else {
            -1
        }
    } else {
        -2
    }
}

pub fn check_ptr_to_val(param: &u8) -> isize {
    if *param < 128 {
        *param as isize
    } else {
        -1
    }
}

pub fn check_option_box_of_val(param: Option<Box<u8>>) -> isize {
    if let Some(val) = param {
        if *val < 128 {
            *val as isize
        } else {
            -1
        }
    } else {
        -2
    }
}

#[allow(clippy::boxed_local)]
pub fn check_box_of_val(param: Box<u8>) -> isize {
    if *param < 128 {
        *param as isize
    } else {
        -1
    }
}

pub fn check_option_box_of_val_ret_val_option_box(
    param: Option<Box<u8>>,
) -> (isize, Option<Box<u8>>) {
    // This match code generated exactly the same code
    // as the if below but looks "simpler"
    match param {
        Some(val) if *val < 128 => (*val as isize, Some(val)),
        Some(val) => (-1, Some(val)),
        None => (-2, None),
    }
    //if let Some(val) = param {
    //    if *val < 128 {
    //        (*val as isize, Some(val))
    //    } else {
    //        (-1, Some(val))
    //    }
    //} else {
    //    (-2, None)
    //}
}

pub fn check_box_of_val_ret_val_option_box(param: Box<u8>) -> (isize, Option<Box<u8>>) {
    match *param {
        128 => (-2, None),
        0..=127 => (*param as isize, Some(param)),
        _ => (-1, Some(param)),
    }
}

pub fn check_box_of_val_ret_val_box(param: Box<u8>) -> (isize, Box<u8>) {
    if *param < 128 {
        (*param as isize, param)
    } else {
        (-1, param)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn check_size_of_u8() {
        assert_eq!(size_of::<u8>(), 1);
    }

    #[test]
    fn none_check_option_val() {
        let result = check_option_val(None);
        assert_eq!(result, -2);
    }

    #[test]
    fn range_check_option_val() {
        let result = check_option_val(Some(0));
        assert_eq!(result, 0);
        let result = check_option_val(Some(127));
        assert_eq!(result, 127);
        let result = check_option_val(Some(128));
        assert_eq!(result, -1);
        let result = check_option_val(Some(255));
        assert_eq!(result, -1);
    }

    #[test]
    fn range_check_val() {
        let result = check_val(0);
        assert_eq!(result, 0);
        let result = check_val(127);
        assert_eq!(result, 127);
        let result = check_val(128);
        assert_eq!(result, -1);
        let result = check_val(255);
        assert_eq!(result, -1);
    }

    #[test]
    fn none_check_option_ptr_to_val() {
        let result = check_option_ptr_to_val(None);
        assert_eq!(result, -2);
    }

    #[test]
    fn range_check_option_ptr_to_val() {
        let mut val = 0;
        let result = check_option_ptr_to_val(Some(&val));
        assert_eq!(result, 0);
        val = 127;
        let result = check_option_ptr_to_val(Some(&val));
        assert_eq!(result, 127);
        val = 128;
        let result = check_option_ptr_to_val(Some(&val));
        assert_eq!(result, -1);
        val = 255;
        let result = check_option_ptr_to_val(Some(&val));
        assert_eq!(result, -1);
    }

    #[test]
    fn range_check_ptr_to_val() {
        let mut val = 0;
        let result = check_ptr_to_val(&val);
        assert_eq!(result, 0);
        val = 127;
        let result = check_ptr_to_val(&val);
        assert_eq!(result, 127);
        val = 128;
        let result = check_ptr_to_val(&val);
        assert_eq!(result, -1);
        val = 255;
        let result = check_ptr_to_val(&val);
        assert_eq!(result, -1);
    }

    #[test]
    fn range_check_option_of_val() {
        let mut val = Box::new(0);
        let result = check_option_box_of_val(Some(val));
        assert_eq!(result, 0);
        val = Box::new(127);
        let result = check_option_box_of_val(Some(val));
        assert_eq!(result, 127);
        val = Box::new(128);
        let result = check_option_box_of_val(Some(val));
        assert_eq!(result, -1);
        val = Box::new(255);
        let result = check_option_box_of_val(Some(val));
        assert_eq!(result, -1);
    }

    #[test]
    fn range_check_box_of_val() {
        let mut val = Box::new(0);
        let result = check_box_of_val(val);
        assert_eq!(result, 0);
        val = Box::new(127);
        let result = check_box_of_val(val);
        assert_eq!(result, 127);
        val = Box::new(128);
        let result = check_box_of_val(val);
        assert_eq!(result, -1);
        val = Box::new(255);
        let result = check_box_of_val(val);
        assert_eq!(result, -1);
    }

    #[test]
    fn range_check_option_box_of_val_ret_val_option_box() {
        let mut val = Box::new(0);
        let (result, obv) = check_option_box_of_val_ret_val_option_box(Some(val));
        assert_eq!(result, 0);
        assert_eq!(obv, Some(Box::new(0)));
        val = Box::new(127);
        let (result, obv) = check_option_box_of_val_ret_val_option_box(Some(val));
        assert_eq!(result, 127);
        assert_eq!(obv, Some(Box::new(127)));
        val = Box::new(255);
        let (result, obv) = check_option_box_of_val_ret_val_option_box(Some(val));
        assert_eq!(result, -1);
        assert_eq!(obv, Some(Box::new(255)));
        let (result, obv) = check_option_box_of_val_ret_val_option_box(None);
        assert_eq!(result, -2);
        assert_eq!(obv, None);
    }

    #[test]
    fn range_check_box_of_val_ret_val_option_box() {
        let mut val = Box::new(0);
        let (result, obv) = check_box_of_val_ret_val_option_box(val);
        assert_eq!(result, 0);
        assert_eq!(obv, Some(Box::new(0)));
        val = Box::new(127);
        let (result, obv) = check_box_of_val_ret_val_option_box(val);
        assert_eq!(result, 127);
        assert_eq!(obv, Some(Box::new(127)));
        val = Box::new(128);
        let (result, obv) = check_box_of_val_ret_val_option_box(val);
        assert_eq!(result, -2);
        assert_eq!(obv, None);
        val = Box::new(255);
        let (result, obv) = check_box_of_val_ret_val_option_box(val);
        assert_eq!(result, -1);
        assert_eq!(obv, Some(Box::new(255)));
    }

    #[test]
    fn range_check_box_of_val_ret_val_box() {
        let mut val = Box::new(0);
        let (result, bv) = check_box_of_val_ret_val_box(val);
        assert_eq!(result, 0);
        assert_eq!(bv, Box::new(0));
        val = Box::new(127);
        let (result, bv) = check_box_of_val_ret_val_box(val);
        assert_eq!(result, 127);
        assert_eq!(bv, Box::new(127));
        val = Box::new(128);
        let (result, bv) = check_box_of_val_ret_val_box(val);
        assert_eq!(result, -1);
        assert_eq!(bv, Box::new(128));
        val = Box::new(255);
        let (result, bv) = check_box_of_val_ret_val_box(val);
        assert_eq!(result, -1);
        assert_eq!(bv, Box::new(255));
    }
}
