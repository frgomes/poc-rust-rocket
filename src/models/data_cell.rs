/*
 * rocket server demo
 *
 * rocket server demo
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: nospam@example.com
 * Generated by: https://openapi-generator.tech
 */

/// DataCell : Data cell.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCell {
    #[serde(rename = "coord")]
    pub coord: crate::models::Coord,
    /// value represented as string.
    #[serde(rename = "value")]
    pub value: String,
}

impl DataCell {
    /// Data cell.
    pub fn new(coord: crate::models::Coord, value: String) -> DataCell {
        DataCell {
            coord,
            value,
        }
    }
}


