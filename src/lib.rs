use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Unit {
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

#[derive(Eq, PartialEq, Debug)]
pub enum UnitValue {
    Byte = 1,
    Kilo = 1 << 10,
    Mega = 1 << 20,
    Giga = 1 << 30,
    Tera = 1 << 40,
}

impl Unit {
    fn to_value(&self) -> UnitValue {
        match self {
            Unit::Byte => UnitValue::Byte,
            Unit::Kilo => UnitValue::Kilo,
            Unit::Mega => UnitValue::Mega,
            Unit::Giga => UnitValue::Giga,
            Unit::Tera => UnitValue::Tera,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Unit::Byte => "",
            Unit::Kilo => "Ki",
            Unit::Mega => "Mi",
            Unit::Giga => "Gi",
            Unit::Tera => "Ti"
        }.to_string();

        write!(f, "{}", string)
    }
}

pub struct Amount {
    bytes: f64,
    unit: Unit
}

impl Amount {
    pub fn new(bytes: f64, unit: Unit) -> Amount {
        Amount { bytes, unit }
    }

    pub fn auto_detect(bytes: f64) -> Amount {
        let scales: [Unit; 5] = [Unit::Byte, Unit::Kilo, Unit::Mega, Unit::Giga, Unit::Tera];
        let mut amount = bytes;
        let mut counter = 0;

        if amount <= 0_f64 {
            return Self::new(0_f64, Unit::Byte)
        }

        while amount > 1.0 && counter < 5 {
            amount = amount / 1024.0;
            counter += 1
        }

        Self::new(bytes, scales[counter - 1])
    }

    pub fn quantity(&self) -> f64 {
        self.bytes / (self.unit.to_value() as u64 as f64)
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }

    pub fn bytes(&self) -> f64 {
        self.bytes
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} {}B", self.quantity(), self.unit.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::{Unit, UnitValue, Amount};

    #[test]
    fn unit_to_value() {
        assert_eq!(Unit::Byte.to_value(), UnitValue::Byte);
        assert_eq!(Unit::Kilo.to_value(), UnitValue::Kilo);
        assert_eq!(Unit::Mega.to_value(), UnitValue::Mega);
        assert_eq!(Unit::Giga.to_value(), UnitValue::Giga);
        assert_eq!(Unit::Tera.to_value(), UnitValue::Tera);
    }

    #[test]
    fn unit_format() {
        assert_eq!(format!("{}", Unit::Byte), String::from(""));
        assert_eq!(format!("{}", Unit::Kilo), String::from("Ki"));
        assert_eq!(format!("{}", Unit::Mega), String::from("Mi"));
        assert_eq!(format!("{}", Unit::Giga), String::from("Gi"));
        assert_eq!(format!("{}", Unit::Tera), String::from("Ti"));
    }

    #[test]
    fn amount_new() {
        let amount = Amount::new(100.0, Unit::Giga);

        assert_eq!(amount.bytes, 100.0);
        assert_eq!(amount.unit, Unit::Giga);
    }

    #[test]
    fn amount_bytes() {
        let amount = Amount::auto_detect(32_f64 * 1024_f64);

        assert_eq!(amount.bytes(), 32768_f64)
    }

    #[test]
    fn amount_quantity() {
        let amount = Amount::auto_detect(32_f64 * 1024_f64);

        assert_eq!(amount.quantity(), 32_f64)
    }

    #[test]
    fn amount_unit() {
        let amount = Amount::auto_detect(32_f64 * 1024_f64);

        assert_eq!(amount.unit(), Unit::Kilo)
    }

    #[test]
    fn amount_auto_detect() {
        assert_eq!(Amount::auto_detect(-1.0).unit, Unit::Byte);
        assert_eq!(Amount::auto_detect(0.0).unit, Unit::Byte);
        assert_eq!(Amount::auto_detect(42.0).unit, Unit::Byte);
        assert_eq!(Amount::auto_detect(2048.0).unit, Unit::Kilo);
        assert_eq!(Amount::auto_detect(1234567.0).unit, Unit::Mega);
        assert_eq!(Amount::auto_detect(1234567890.0).unit, Unit::Giga);
        assert_eq!(Amount::auto_detect(1234567890123.0).unit, Unit::Tera);
    }

    #[test]
    fn amount_display() {
        assert_eq!(format!("{}", Amount::auto_detect(42.0)), "42.0 B");
        assert_eq!(format!("{}", Amount::auto_detect(200124.42)), "195.4 KiB");
    }
}
