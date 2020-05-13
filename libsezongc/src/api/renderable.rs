/// Renderable node
pub trait Renderable {
    /// The namespace that the node belongs to
    fn namespace() -> &'static str;
    /// The name of node
    fn name() -> &'static str;
}
