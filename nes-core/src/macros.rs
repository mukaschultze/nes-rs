#[macro_export]
macro_rules! join_bytes {
    ($hh:expr, $ll:expr) => {
        (($hh as u16) << 8) | ($ll as u16)
    };
}

#[macro_export]
macro_rules! high_byte {
    ($value:expr) => {
        (($value >> 8) & 0xFF) as u8
    };
}

#[macro_export]
macro_rules! low_byte {
    ($value:expr) => {
        ($value & 0xFF) as u8
    };
}

#[macro_export]
macro_rules! page_crossed {
    ($left:expr,$right:expr) => {
        $left & 0xFF00 != $right & 0xFF00
    };
}

#[macro_export]
macro_rules! unchecked_add {
    ($left:expr,$right:expr) => ((std::num::Wrapping($left) + std::num::Wrapping($right)).0);
    ($left:expr,$right:expr,$($other:expr),+) => (unchecked_add!(unchecked_add!($left, $right), $($other),+));
}

#[macro_export]
macro_rules! unchecked_sub {
    ($left:expr,$right:expr) => ((std::num::Wrapping($left) - std::num::Wrapping($right)).0);
    ($left:expr,$right:expr,$($other:expr),+) => (unchecked_sub!(unchecked_sub!($left, $right), $($other),+));
}

#[macro_export]
macro_rules! rel_addr {
    ($address:expr, $displacement:expr) => {
        ($address as i32 + $displacement as i32) as u16
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn join_bytes() {
        assert_eq!(join_bytes!(0x12, 0x34), 0x1234);
    }

    #[test]
    fn high_byte() {
        assert_eq!(high_byte!(0x1234), 0x12);
    }

    #[test]
    fn low_byte() {
        assert_eq!(low_byte!(0x1234), 0x34);
    }

    #[test]
    fn cross_page() {
        assert_eq!(page_crossed!(0x0123, 0x234), true);
        assert_eq!(page_crossed!(0x0123, 0x134), false);
    }

    #[test]
    fn unchecked_add() {
        assert_eq!(unchecked_add!(15u8, 15u8), 30u8);
        assert_eq!(unchecked_add!(15u8, 15u8, 15u8), 45u8);
        assert_eq!(unchecked_add!(150u8, 150u8), 44u8);
        assert_eq!(unchecked_add!(100u8, 100u8, 100u8), 44u8);
    }

    #[test]
    fn unchecked_sub() {
        assert_eq!(unchecked_sub!(30u8, 15u8), 15u8);
        assert_eq!(unchecked_sub!(90u8, 30u8, 30u8), 30u8);
        assert_eq!(unchecked_sub!(100u8, 150u8), 206u8);
        assert_eq!(unchecked_sub!(100u8, 50u8, 100u8), 206u8);
    }

    #[test]
    fn rel_addr() {
        assert_eq!(rel_addr!(0x1234, 10), 0x123E);
        assert_eq!(rel_addr!(0x1234, -10), 0x122A);
    }
}
