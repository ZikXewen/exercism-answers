#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    let mut x = number.iter().rev().enumerate().try_fold(0, |cr, (i, &x)| {
        if x >= from_base {
            Err(Error::InvalidDigit(x))
        } else {
            Ok(cr + x * from_base.pow(i as u32))
        }
    })?;
    let mut vec = vec![];
    loop {
        vec.push(x % to_base);
        x /= to_base;
        if x == 0 {
            vec.reverse();
            return Ok(vec);
        }
    }
}
