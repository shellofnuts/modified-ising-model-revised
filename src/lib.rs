// List of things that are needed
// 1. Magnetic moment vector
// 2. Node struct that holds magnetic moment, maybe energy?
// 3. Node should be able to calculate its energy, should it know its neighbours or be given them?
// 4. Lattice object - initially for a square matrix. Once that is done, try to go up to different basis sets.
// 5. Implement metropolis algorithm that loops over the matrix.
// 6. File IO to print results?
// 

mod hamiltonian;
mod magnetic_node;

use std::fmt::Debug;

pub struct Lattice {
    pub lattice: Vec<Vec<magnetic_node::Node>>
}

impl Lattice {
    pub fn new((i, j): (usize, usize)) -> Lattice {
        let lattice = vec![vec![magnetic_node::Node::new(); i]; j];
        Lattice {lattice}
    }

    pub fn energy(&self, i: usize, j: usize, hamiltonian: hamiltonian::Hamiltonian) -> f64 {
        let mut align = 0f64;
        let neighbours = vec![(1, 0), (-1, 0), (0, 1), (0, 1)];

        let Ok(i) = i.try_into() else {
            panic!("Lattice too large.");
        };
        let Ok(j) = j.try_into() else {
            panic!("Lattice too large.");
        };

        let ij_moment = self.moment(i, j);

        for (jj, ii) in neighbours {
            align += -1f64 * (1f64 - hamiltonian.exchange_anisotropy) * ( 
                ij_moment.x_vec * self.moment(ii, jj).x_vec +
                ij_moment.y_vec * self.moment(ii, jj).y_vec
            )
        }
        align += -1f64 * hamiltonian.axis_anisotropy * (ij_moment.z_vec * ij_moment.z_vec);
        align *= hamiltonian.interaction_energy;
        align
    }

    pub fn moment(&self, mut i: i32, mut j: i32) -> magnetic_node::MagneticMoment {
        let Ok(i_len) = self.lattice[0].len().try_into() else {
            panic!("Lattice too large.");
        };
        let Ok(j_len) = self.lattice.len().try_into() else {
            panic!("Lattice too large.");
        };
        
        if i < 0 {
            i += i_len
        }
        else if i > i_len {
            i = i % i_len;
        }
        if j < 0 {
            j += j_len;
        }
        else if j > j_len {
            j = j % j_len;
        }
        self.lattice[j as usize][i as usize].moment
    }
}

impl Debug for Lattice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lat = std::string::String::new();
        for i in self.lattice.iter() {
            for j in i {
                lat.push_str("{j.moment.z_vec}, ");
            }
        }
        write!(f, "")
    }
}