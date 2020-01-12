#![allow(dead_code)]
//#![allow(unused_assignments)]
//#![allow(unused_variables)]
mod combat_predictor;
#[macro_use]
extern crate lazy_static;

mod combat_unit;
mod generated_enums;
#[macro_use]
extern crate cached;

mod game_info;
use pyo3::prelude::*;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[pymodule]
fn sc2_helper(_py: Python, m: &PyModule) -> PyResult<()> {
    //    m.add_class::<combat_unit::CombatUnits>()?;
    m.add_class::<combat_predictor::CombatPredictor>()?;
    m.add_class::<combat_unit::CombatUnit>()?;
    m.add_class::<combat_predictor::CombatSettings>()?;
    Ok(())
}
