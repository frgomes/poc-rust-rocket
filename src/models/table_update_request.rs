/*
 * rocket server demo
 *
 * rocket server demo
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: nospam@example.com
 * Generated by: https://openapi-generator.tech
 */

/// TableUpdateRequest : Request for updating a table.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableUpdateRequest {
    /// Semantic versioning information (MAJOR.MINOR.PATCH).
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "table")]
    pub table: crate::models::DataTable,
}

impl TableUpdateRequest {
    /// Request for updating a table.
    pub fn new(version: String, table: crate::models::DataTable) -> TableUpdateRequest {
        TableUpdateRequest {
            version,
            table,
        }
    }
}


