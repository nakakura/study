use carboxyl::*;

pub struct Delivery;
pub struct Sale;

pub struct Outputs {
    delivery: Signal<Delivery>,
    preset_lcd: Signal<f64>,
    sale_cost_lcd: Signal<f64>,
    sale_quantity_lcd: Signal<f64>,
    price_lcd1: Signal<f64>,
    price_lcd2: Signal<f64>,
    price_lcd3: Signal<f64>,
    beep: Stream<()>,
    sale_complete: Stream<Sale>,
}

impl Outputs {
    fn new(delivery: Signal<Delivery>,
           preset_lcd: Signal<f64>,
           sale_cost_lcd: Signal<f64>,
           sale_quantity_lcd: Signal<f64>,
           price_lcd1: Signal<f64>,
           price_lcd2: Signal<f64>,
           price_lcd3: Signal<f64>,
           beep: Stream<()>,
           sale_complete: Stream<Sale>) -> Self {
        Outputs {
            delivery: delivery,
            preset_lcd: preset_lcd,
            sale_cost_lcd: sale_cost_lcd,
            sale_quantity_lcd: sale_quantity_lcd,
            price_lcd1: price_lcd1,
            price_lcd2: price_lcd2,
            price_lcd3: price_lcd3,
            beep: beep,
            sale_complete: sale_complete
        }
    }

    pub fn set_delivery(self, delivery: Signal<Delivery>) -> Self {
        Outputs {
            delivery: delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: self.beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_preset_lcd(self, preset_lcd: Signal<f64>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: self.beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_sale_cost_lcd(self, sale_cost_lcd: Signal<f64>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: self.beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_sale_quantity_lcd(self, sale_quantity_lcd: Signal<f64>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: self.beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_price_lcd1(self, price_lcd1: Signal<f64>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: self.beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_price_lcd2(self, price_lcd2: Signal<f64>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: self.beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_price_lcd3(self, price_lcd3: Signal<f64>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: price_lcd3,
            beep: self.beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_beep(self, beep: Stream<()>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: beep,
            sale_complete: self.sale_complete
        }
    }

    pub fn set_sale_complete(self, sale_complete: Stream<Sale>) -> Self {
        Outputs {
            delivery: self.delivery,
            preset_lcd: self.preset_lcd,
            sale_cost_lcd: self.sale_cost_lcd,
            sale_quantity_lcd: self.sale_quantity_lcd,
            price_lcd1: self.price_lcd1,
            price_lcd2: self.price_lcd2,
            price_lcd3: self.price_lcd3,
            beep: self.beep,
            sale_complete: sale_complete
        }
    }
}