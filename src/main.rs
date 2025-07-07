mod hamiltonian;

fn main() {
    // What structure do I want?
    // I suppose I want to set the parameters here?
    // All of the energy parameter should be set in the Hamiltonian. Just the lattice should hold a single Hamiltonian struct. It is immutable anyway.
    // Should the lattice simulate itself or be a parameter?
    // I feel like it should simulate itself.
    // But then how do I implement the metropolis cooling effect?
    // I guess the simulate should _properly_ randomly flip a number of nodes such that the expected value is that the whole lattice is "simulated".
    // Or a cooling parameter should be gien when it is new, so it knows how fast to cool down
    // The issue then occurs with cooling. Each sub-matrix could find a different local minimum. So maybe it has to be linear?
    // Or the temperature range is split up as per the previous implementation?

    let interaction_energy: f64 = 1e6;
    let exchange_energy: f64 = 0.0;
    let axis_anisotropy: f64 = 1.0;
    
    let hamiltonian = hamiltonian::Hamiltonian::new(interaction_energy, exchange_energy, axis_anisotropy);
    
    let metropolis = modified_ising_model_revised::Metropolis::new();

    let mut lattice = modified_ising_model_revised::Lattice::new((10, 10));

}
