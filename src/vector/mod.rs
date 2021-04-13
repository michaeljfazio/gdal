//! GDAL Vector Data
//!
//! ## Reading
//!
//! ```
//! use std::path::Path;
//! use gdal::Dataset;
//!
//! let mut dataset = Dataset::open(Path::new("fixtures/roads.geojson")).unwrap();
//! let layer = dataset.layer(0).unwrap();
//! for feature in layer.features() {
//!     let highway_field = feature.field("highway").unwrap().unwrap();
//!     let geometry = feature.geometry();
//!     println!("{} {}", highway_field.into_string().unwrap(), geometry.wkt().unwrap());
//! }
//! ```

mod defn;
mod feature;
pub mod gdal_to_geo;
pub mod geo_to_gdal;
mod geometry;
mod layer;
mod ops;
pub mod sql;

pub use defn::{Defn, Field, FieldIterator};
pub use feature::{Feature, FieldValue, FieldValueIterator};
pub use gdal_sys::{OGRFieldType, OGRwkbGeometryType};
pub use geometry::Geometry;
pub use layer::{FeatureIterator, FieldDefn, Layer, LayerCaps};
pub use ops::GeometryIntersection;

#[cfg(test)]
mod vector_tests;
