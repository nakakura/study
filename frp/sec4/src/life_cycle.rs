use carboxyl::*;

use inputs::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Fuel {
    ONE, TWO, THREE
}

pub struct End;

pub struct LifeCycle {
    start: Stream<Fuel>,
    end: Stream<End>,
    fill_active: Signal<Option<Fuel>>,
}

impl  LifeCycle {
    fn when_lifted(nozzle: Stream<UpDown>, nozzle_fuel: Fuel) -> Stream<Fuel> {
        nozzle.filter(|u| u == &UpDown::UP).map(move |u| nozzle_fuel)
    }

    fn when_set_down(nozzle: Stream<UpDown>, nozzle_fuel: Fuel, fill_active: Signal<Option<Fuel>>) {
    }
}

