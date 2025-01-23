pub mod hosting;
pub mod service;
use crate::front_of_house::hosting::{add_to_waitlist, seat_at_table};

pub fn eat_at_restaurant() {
    // use Directly
    add_to_waitlist();
    seat_at_table();

    service::take_order();

    super::back_of_house::cooking::cooking_order();
    //  Parenet root to use good way
    // if use stuct and enum fully path
    service::serve_order();
    // Root
    crate::front_of_house::service::take_payment();
}
