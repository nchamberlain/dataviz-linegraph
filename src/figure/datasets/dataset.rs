use super::{
    areachartdataset::AreaChartDataset, bardataset::BarDataset,
    cartesiangraphdataset::CartesianDataset, scattergraphdataset::ScatterGraphDataset,
    linegraphdataset::LineGraphDataset,
};
use crate::figure::utilities::{linetype::LineType, scatterdottype::ScatterDotType,};


/// A trait for managing datasets used in different types of charts or graphs.
pub trait Dataset {
    /// Retrieves all points in the dataset as a vector of `(x, y)` tuples.
    ///
    /// # Returns
    /// A vector of `(f64, f64)` representing the data points in the dataset.
    fn get_points(&self) -> Vec<(f64, f64)>;

    /// Adds a single point to the dataset.
    ///
    /// # Parameters
    /// - `point`: A tuple `(f64, f64)` representing the x and y coordinates of the point to add.
    fn add_point(&mut self, point: (f64, f64));
}

impl Dataset for BarDataset {
    /// Implementation of the `Dataset` trait for `BarDataset`.
    ///
    /// - `get_points`: Returns the bar data as `(x, y)` pairs.
    /// - `add_point`: Adds a new `(x, y)` pair to the bar dataset.
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.data.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.data.push(point);
    }
}

impl Dataset for CartesianDataset {
    /// Implementation of the `Dataset` trait for `CartesianDataset`.
    ///
    /// - `get_points`: Returns the Cartesian data as `(x, y)` pairs.
    /// - `add_point`: Adds a new `(x, y)` pair to the Cartesian dataset.
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }
}

impl Dataset for ScatterGraphDataset {
    /// Implementation of the `Dataset` trait for `ScatterGraphDataset`.
    ///
    /// - `get_points`: Returns the scatter graph data as `(x, y)` pairs.
    /// - `add_point`: Adds a new `(x, y)` pair to the scatter graph dataset.
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }
}

impl Dataset for AreaChartDataset {
    /// Implementation of the `Dataset` trait for `AreaChartDataset`.
    ///
    /// - `get_points`: Returns the area chart data as `(x, y)` pairs.
    /// - `add_point`: Adds a new `(x, y)` pair to the area chart dataset.
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }
}

impl Dataset for LineGraphDataset {
    /// Implementation of the `Dataset` trait for `LineGraphDataset`.
    ///
    /// - `get_points`: Returns the LineGraph data as `(x, y)` pairs.
    /// - `add_point`: Adds a new `(x, y)` pair to the LineGraph dataset.
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_dataset() {
        let mut dataset = BarDataset::new("Test Bar", [255, 0, 0]);
        dataset.add_point((1.0, 2.0));
        dataset.add_point((3.0, 4.0));
        let points = dataset.get_points();
        assert_eq!(points, vec![(1.0, 2.0), (3.0, 4.0)]);
    }

    #[test]
    fn test_cartesian_dataset() {
        let mut dataset = CartesianDataset::new([0, 255, 0], "Test Cartesian", LineType::Solid);
        dataset.add_point((5.0, 6.0));
        let points = dataset.get_points();
        assert_eq!(points, vec![(5.0, 6.0)]);
    }

    #[test]
    fn test_scatter_graph_dataset() {
        let mut dataset = ScatterGraphDataset::new([0, 0, 255], "Test Scatter", ScatterDotType::Circle(5));
        dataset.add_point((7.0, 8.0));
        let points = dataset.get_points();
        assert_eq!(points, vec![(7.0, 8.0)]);
    }

    #[test]
    fn test_area_chart_dataset() {
        let mut dataset = AreaChartDataset::new([255, 255, 0],"Test Area", 0.5);
        dataset.add_point((9.0, 10.0));
        let points = dataset.get_points();
        assert_eq!(points, vec![(9.0, 10.0)]);
    }

    #[test]
    fn test_line_graph_dataset() {
        let mut dataset = LineGraphDataset::new([0, 255, 255], "Test Line",  LineType::Dashed(4));
        dataset.add_point((11.0, 12.0));
        let points = dataset.get_points();
        assert_eq!(points, vec![(11.0, 12.0)]);
    }
}