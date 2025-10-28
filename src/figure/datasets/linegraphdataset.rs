use crate::figure::utilities::linetype::LineType;

/// A dataset for line graphs, first draft

pub struct LineGraphDataset {
    /// A collection of `(x, y)` data points for the line graph.
    pub points: Vec<(f64, f64)>, //should these be ints?
    /// Color of the line in RGB format.
    pub color: [u8; 3],
    /// Label for the dataset, used in legends or annotations.
    pub label: String,
    /// Style of the line (solid, dashed, dotted).
    pub line_type: LineType,
}

impl LineGraphDataset {
    /// Creates a new `LineGraphDataset` instance with the specified appearance and metadata.
    ///
    /// # Parameters
    /// - `color`: The RGB color of the line.
    /// - `label`: A descriptive label for the dataset.
    /// - `line_type`: The style of the line (`LineType`).
    ///
    /// # Returns
    /// A new `LineGraphDataset` instance with an empty list of points.
    ///
    /// # Example
    /// ```rust
    /// use crate::figure::utilities::linetype::LineType;
    ///
    /// let dataset = LineGraphDataset::new([0, 128, 255], "Temperature", LineType::Dashed(10));
    /// ```
    pub fn new(color: [u8; 3], label: &str, line_type: LineType) -> Self {
        Self {
            points: Vec::new(),
            color,
            label: label.to_string(),
            line_type,
        }
    }
}
