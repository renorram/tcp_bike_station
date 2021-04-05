use std::cmp::{Eq, PartialEq};
use std::time::{Duration, Instant};

const TIME_LIMIT: Duration = Duration::from_secs(30 * 60);

#[derive(Debug, Eq, PartialEq)]
pub enum BikeRentalError {
    AlreadyRegistered,
    RentRegisterNotFound,
}

type BikeRentalResult<T> = Result<T, BikeRentalError>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct RentRegister {
    pub(crate) id_person: String,
    pub(crate) id_station: String,
    rented_time: Instant,
}

impl RentRegister {
    pub fn new(id_person: &str, id_station: &str) -> Self {
        RentRegister {
            id_person: id_person.to_string(),
            id_station: id_station.to_string(),
            rented_time: Instant::now(),
        }
    }

    fn time_passed(&self) -> Duration {
        self.rented_time.elapsed()
    }
}

#[derive(Clone)]
pub(crate) struct BikeRental {
    time_limit: Duration,
    registers: Vec<RentRegister>,
}

// impl Clone for Vec<RentRegister> {
//     fn clone(&self) -> Self {
//         self.iter().map(|r| r.clone()).into()
//     }
// }

impl BikeRental {
    pub fn new(registers: Vec<RentRegister>, time_limit: Option<Duration>) -> Self {
        let time_limit = match time_limit {
            Some(duration) => duration,
            None => TIME_LIMIT
        };

        BikeRental { registers, time_limit }
    }

    pub fn register_rent(
        &mut self,
        id_person: &str,
        id_station: &str,
    ) -> BikeRentalResult<&RentRegister> {
        if let Some(_) = self.registers.iter().find(|r| r.id_person.eq(id_person)) {
            return Err(BikeRentalError::AlreadyRegistered);
        }

        self.registers.push(RentRegister::new(id_person, id_station));

        Ok(self.registers.last().unwrap())
    }

    pub fn finish_rent(&mut self, id_person: &str) -> BikeRentalResult<String> {
        if let Some(position) = self.registers.iter().position(|r| r.id_person.eq(id_person)) {
            let mut message: String = String::from("Rent finished succesfully.");

            // check if the renting time passed the free time
            if self.registers[position].time_passed().ge(&self.time_limit) {
                message.push_str(" You own $5.");
            }

            self.registers.remove(position);
            return Ok(message);
        }

        return Err(BikeRentalError::RentRegisterNotFound);
    }
}

impl ToString for BikeRentalError {
    fn to_string(&self) -> String {
        match self {
            Self::AlreadyRegistered => String::from("Already registered error."),
            Self::RentRegisterNotFound => String::from("Renter register not found."),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::station::{BikeRental, RentRegister, BikeRentalError};
    use std::time::Duration;

    #[test]
    fn test_can_rent() {
        let mut rental = BikeRental::new(Vec::new(), None);
        let rent_register = rental.register_rent("id", "station");

        assert!(rent_register.is_ok());

        let rent_register = rent_register.unwrap();
        assert_eq!(rent_register.id_person, "id");
        assert_eq!(rent_register.id_station, "station")
    }

    #[test]
    fn test_charge_rent() {
        let mut rental = BikeRental::new(Vec::new(), Some(Duration::from_millis(1)));
        let rent_register = rental.register_rent("id", "station");

        assert!(rent_register.is_ok());

        std::thread::sleep(Duration::from_millis(2));
        let finished_rent = rental.finish_rent("id");
        assert!(finished_rent.is_ok());
        assert!(finished_rent.unwrap().contains("You own $5."))
    }

    #[test]
    fn test_cant_rent_twice() {
        let mut rental = BikeRental::new(vec![RentRegister::new("id", "station")], None);
        let rent_register = rental.register_rent("id", "station");

        assert!(rent_register.is_err());
        assert_eq!(Err(BikeRentalError::AlreadyRegistered), rent_register);
    }

    #[test]
    fn test_cant_finish_nonexistent_rent() {
        let mut rental = BikeRental::new(Vec::new(), None);
        let finished_rent = rental.finish_rent("id");

        assert!(finished_rent.is_err());
        assert_eq!(Err(BikeRentalError::RentRegisterNotFound), finished_rent);
    }
}
