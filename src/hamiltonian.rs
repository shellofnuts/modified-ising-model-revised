#[derive(Debug)]
pub struct Hamiltonian {
    pub interaction_energy: f64,
    pub axis_anisotropy : f64,
    pub exchange_anisotropy : f64,
}

impl Hamiltonian {
    pub fn new(energy: f64, axis: f64, exchange: f64) -> Hamiltonian {
        Hamiltonian { interaction_energy: energy, axis_anisotropy: axis, exchange_anisotropy: exchange }
    }
    
}