/// Represents the style of a line in a graph or chart.
#[derive(Clone)]
pub enum LineType {
    /// A solid line with no gaps.
    Solid,
    /// A thick solid line with no gaps 
    /// - The `u32` value specifies the thickness of the line
    SolidThick(u32),
    /// A dashed line with configurable dash length.
    /// - The `u32` value specifies the length of each dash in pixels.
    Dashed(u32),
    /// A dotted line with configurable dot spacing.
    /// - The `u32` value specifies the spacing between dots in pixels.
    Dotted(u32),
    /// A line of squares with configurable dot spacing
    /// - The `u32` value specifies the spacing between the squares
    /// - The `u32` value specifies the size of each square
    Squared(u32, u32),
}
