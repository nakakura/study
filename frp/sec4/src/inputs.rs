use carboxyl::*;

pub struct UpDown;
pub struct Key;

pub struct Inputs {}

impl Inputs {
    fn new(nozzle1: Stream<UpDown>, nozzle2: Stream<UpDown>, nozzle3: Stream<UpDown>, keypad: Stream<Key>, fuel_pulses: Stream<isize>,
    calibration: Signal<f64>, price1: Signal<f64>, price2: Signal<f64>, price3: Signal<f64>, clear_sale: Stream<()>) {

    }
}