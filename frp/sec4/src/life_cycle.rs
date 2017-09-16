use carboxyl::*;

use inputs::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Fuel {
    ONE, TWO, THREE
}

#[derive(Clone, Copy, PartialEq)]
pub struct End;

pub struct LifeCycle {
    start: Stream<Fuel>,
    end: Stream<End>,
    fill_active: Signal<Option<Fuel>>,
}

impl  LifeCycle {
    fn when_lifted(nozzle: &Stream<UpDown>, nozzle_fuel: Fuel) -> Stream<Fuel> {
        nozzle.filter(|u| u == &UpDown::UP).map(move |u| nozzle_fuel)
    }

    fn when_set_down(nozzle_stream: &Stream<UpDown>, nozzle_fuel: Fuel, fill_active: &Signal<Option<Fuel>>) -> Stream<End> {
        fill_active.snapshot(&nozzle_stream, move |nozzle, b: UpDown| {
            if b.eq(&UpDown::DOWN) && nozzle.eq(&Some(nozzle_fuel)) {
                Some(End)
            } else {
                None
            }
        }).filter_some()
    }

    pub fn start(nozzle1: Stream<UpDown>, nozzle2: Stream<UpDown>, nozzle3: Stream<UpDown>) {
        let nozzle_stream = LifeCycle::when_lifted(&nozzle1, Fuel::ONE)
            .merge(&LifeCycle::when_lifted(&nozzle2, Fuel::TWO))
            .merge(&LifeCycle::when_lifted(&nozzle3, Fuel::THREE));

        /*
        let fill_active = Signal::cyclic(|x| {

        })
        *;
    }
}

