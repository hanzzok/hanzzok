/// A platform where the sezong documents render to
pub trait Platform {
    /// The intermediate output type
    type MidOutput;
    /// The final output type
    type Output;

    /// The name of platform
    fn name() -> String;
    /// Combines intermediate outputs to final output
    fn combine(mid_results: &[Self::MidOutput]) -> Self::Output;
}
