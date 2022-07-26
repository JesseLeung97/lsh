use crate::config::Config;
use rand::{Rng, thread_rng};
use std::f32::consts::E;

/// The normal vector to an n dimensional hyperplane
/// Used to reduce a single digit of a vector to the range 0..1
pub struct Hyperplane {
    pub vector: Vec<f32>
}

/// A set of n n dimensional hyperplane normal vectors
/// Used to reduce every digit of a vector to the range 0..1
/// One set is used to generate a vector's hashcode for a particular hashtable
pub struct HyperplaneSet {
    pub set: Vec<Hyperplane>
}

/// A collection of k hyperplane sets
/// Used to generate k hashcodes for each vector
pub struct HyperplaneCollection {
    pub collection: Vec<HyperplaneSet>
}

/// Contains the methods used to generate and manipulate the hyperplane struct
impl Hyperplane {
    fn new_hyperplane(config: &Config) -> Hyperplane {
        Hyperplane { vector: (0..config.dimension).map(|_| LSH::random_number()).collect() }
    }

    fn new_hyperplane_set(config: &Config) -> HyperplaneSet {
        HyperplaneSet { set: (0..config.dimension).map(|_| Self::new_hyperplane(config)).collect()}
    }

    pub fn new_hyperplane_collection(config: &Config, table_count: u32) -> HyperplaneCollection {
        HyperplaneCollection { collection: (0..table_count).map(|_| Self::new_hyperplane_set(config)).collect() }
    }
}

/// A collection of utility methods for performing locality sensitive hashing
pub struct LSH;
impl LSH {
    pub fn random_number() -> f32 {
        let u_1: f32 = thread_rng().gen_range(0f32..=1f32) as f32;
        let v_2: f32 = thread_rng().gen_range(0f32..=1f32) as f32;
        let u_2: f32 = (2.0 * v_2 - 1.0) * (2.0 * E.powf(-1.0)).sqrt();
        if u_2 * u_2 > -4.0 * u_1 * u_1 * u_1.log10() {
            return Self::random_number();
        }
        return u_2 / u_1;
    }
    
    pub fn dot_product(vector: &Vec<f32>, hyperplane_normal: &Vec<f32>) -> f32 {
        vector.iter()
            .zip(hyperplane_normal)
            .map(|(x, y)| x * y)
            .sum()
    }
    
    pub fn normalize_dot_product(dot_product: f32) -> bool {
        if dot_product >= 0.0 { true } else { false } 
    }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;
    use test_case::test_case;
    use super::*;
    
    #[test]
    fn random_number_test() {
        let actual = LSH::random_number();
        let range = 0.0..=1.0;
        assert!(range.contains(&actual));
    }
    
    #[test_case(0.5, vec![1.0], vec![0.5])]
    #[test_case(0.5, vec![1.0, 0.25], vec![0.25, 1.0])]
    #[test_case(1.5428, vec![0.3, 0.47, 0.11, 0.89, 0.70, 0.63], vec![0.29, 0.45, 0.91, 0.54, 0.57, 0.42])]
    fn dot_product_test_equal(test_dot_product: f32, test_vec: Vec<f32>, test_hyperplane_normal: Vec<f32>) {
        let actual = LSH::dot_product(&test_vec, &test_hyperplane_normal);
        assert_eq!(test_dot_product, actual);
    }
    
    #[test_case(true, 10.0)]
    #[test_case(true, 0.0)]
    #[test_case(false, -10.0)]
    fn normalize_dot_product_test(test_normalized_dot_product: bool, test_dot_product: f32) {
        let actual = LSH::normalize_dot_product(test_dot_product);
        assert_eq!(test_normalized_dot_product, actual);
    }
}










