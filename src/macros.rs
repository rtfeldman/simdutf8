/// possible compiler hint that a branch is likely
#[cfg(feature = "hints")]
#[macro_export]
macro_rules! likely {
    ($e:expr) => {
        std::intrinsics::likely($e)
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(feature = "hints")]
#[macro_export]
macro_rules! unlikely {
    ($e:expr) => {{
        std::intrinsics::unlikely($e)
    }};
}

/// possible compiler hint that a branch is likely
#[cfg(not(feature = "hints"))]
#[macro_export]
macro_rules! likely {
    ($e:expr) => {
        $e
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(not(feature = "hints"))]
#[macro_export]
macro_rules! unlikely {
    ($e:expr) => {
        $e
    };
}

/// static cast to an i8
#[macro_export]
macro_rules! static_cast_i8 {
    ($v:expr) => {
        mem::transmute::<_, i8>($v)
    };
}

/// static cast to an i32
#[macro_export]
macro_rules! static_cast_i32 {
    ($v:expr) => {
        mem::transmute::<_, i32>($v)
    };
}

/// static cast to an u32
#[macro_export]
macro_rules! static_cast_u32 {
    ($v:expr) => {
        mem::transmute::<_, u32>($v)
    };
}

/// static cast to an i64
#[macro_export]
macro_rules! static_cast_i64 {
    ($v:expr) => {
        mem::transmute::<_, i64>($v)
    };
}

/// static cast to an i64
#[macro_export]
macro_rules! static_cast_i128 {
    ($v:expr) => {
        mem::transmute::<_, i128>($v)
    };
}

/// static cast to an u64
#[macro_export]
macro_rules! static_cast_u64 {
    ($v:expr) => {
        mem::transmute::<_, u64>($v)
    };
}

/// check_bytes() strategy
#[macro_export]
macro_rules! check_bytes {
    ($t:ident) => {
        pub(crate) unsafe fn check_bytes(current: $t, previous: &mut Utf8CheckingState<$t>) {
            if likely!(Self::is_ascii(current)) {
                previous.error = Self::check_eof(previous.error, previous.incomplete)
            } else {
                let prev1 = Self::prev1(current, previous.prev);
                let sc = Self::check_special_cases(current, prev1);
                previous.error = Self::or(
                    previous.error,
                    Self::check_multibyte_lengths(current, previous.prev, sc),
                );
                previous.incomplete = Self::is_incomplete(current);
            }
            previous.prev = current
        }
    };
}

/// check_bytes() strategy
#[macro_export]
macro_rules! validate_utf8_simd {
    () => {
        pub(crate) fn validate_utf8_simd(
            input: &[u8],
        ) -> std::result::Result<&str, crate::Utf8Error> {
            const SIMDINPUT_LENGTH: usize = 64;
            unsafe {
                let len = input.len();
                let mut state = SimdInput::new_utf8_checking_state();
                let lenminus64: usize = if len < 64 { 0 } else { len as usize - 64 };
                let mut idx: usize = 0;

                while idx < lenminus64 {
                    /*
                    #ifndef _MSC_VER
                      __builtin_prefetch(buf + idx + 128);
                    #endif
                     */
                    let input = SimdInput::new(input.get_unchecked(idx as usize..));
                    input.check_utf8(&mut state);
                    idx += SIMDINPUT_LENGTH;
                }

                if idx < len {
                    let mut tmpbuf: [u8; SIMDINPUT_LENGTH] = [0x20; SIMDINPUT_LENGTH];
                    tmpbuf
                        .as_mut_ptr()
                        .copy_from(input.as_ptr().add(idx), len as usize - idx);
                    let input = SimdInput::new(&tmpbuf);

                    input.check_utf8(&mut state);
                }
                SimdInput::check_eof(&mut state);
                if SimdInput::check_utf8_errors(&state) {
                    Err(crate::Utf8Error {})
                } else {
                    Ok(std::str::from_utf8_unchecked(input))
                }
            }
        }
    };
}
