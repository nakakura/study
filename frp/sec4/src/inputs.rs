use carboxyl::*;

#[derive(Clone, Copy, PartialEq)]
pub enum UpDown {
    UP, DOWN
}
pub struct Key;

pub struct Inputs {
    nozzle1: Stream<UpDown>,
    nozzle2: Stream<UpDown>,
    nozzle3: Stream<UpDown>,
    keypad: Stream<Key>,
    fuel_pulses: Stream<isize>,
    calibration: Signal<f64>,
    price1: Signal<f64>,
    price2: Signal<f64>,
    price3: Signal<f64>,
    clear_sale: Stream<()>,
}

impl Inputs {
    fn new(nozzle1: Stream<UpDown>, nozzle2: Stream<UpDown>, nozzle3: Stream<UpDown>, keypad: Stream<Key>, fuel_pulses: Stream<isize>,
    calibration: Signal<f64>, price1: Signal<f64>, price2: Signal<f64>, price3: Signal<f64>, clear_sale: Stream<()>) -> Self {
        Inputs {
            nozzle1: nozzle1,
            nozzle2: nozzle2,
            nozzle3: nozzle3,
            keypad: keypad,
            fuel_pulses: fuel_pulses,
            calibration: calibration,
            price1: price1,
            price2: price2,
            price3: price3,
            clear_sale: clear_sale
        }
    }
}