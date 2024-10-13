use nalgebra_glm::Mat3;

/// Compute the spectral norm of a matrix. That is, the square root of the largest eigenvalue of
/// the matrix's transpose times the matrix itself.
///
/// # Arguments
/// * `m` - The matrix to compute the spectral norm of.
pub fn compute_spectral_norm(m: &Mat3) -> f32 {
    // Compute the eigenvalues of the matrix's transpose times the matrix itself.
    let eigenvalue_list = (m.transpose() * m).eigenvalues();

    // Find the largest eigenvalue.
    if let Some(eigenvalue_list) = eigenvalue_list {
        eigenvalue_list
            .iter()
            .map(|x| x.abs())
            .fold(0.0, f32::max)
            .sqrt()
    } else {
        0.0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_spectral_norm() {
        use super::compute_spectral_norm;
        use nalgebra_glm::Mat3;

        assert_eq!(compute_spectral_norm(&Mat3::identity()), 1.0f32);
        assert_eq!(
            compute_spectral_norm(&Mat3::new(1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0)),
            3.0f32
        );

        assert!(
            (compute_spectral_norm(&Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))
                - 16.848103f32)
                < 1e-3f32
        );
    }
}
