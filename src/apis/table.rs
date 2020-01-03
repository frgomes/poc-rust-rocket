#[post("/table_update", format = "json", data = "<info>")]
pub fn table_update(info: rocket_contrib::json::Json<crate::models::TableUpdateRequest>) -> rocket_contrib::json::JsonValue {
    let body = 
        crate::models::TableUpdateResponse {
            version: "1.0.0".to_owned(),
            table: crate::models::ErrorTable {
                name: "table".to_owned(),
                cells: vec! {
                    crate::models::ErrorCell {
                        coord: crate::models::Coord { row: 1, col: 2 },
                        error: crate::models::Error { code: 10123, message: "nuclear hazard detected".to_owned() },
                    },
                },
            }
        };
    rocket_contrib::json!(body)
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    async fn table_update() {
        let input =
            crate::models::TableUpdateRequest {
                version: "1.0.0".to_owned(),
                table: crate::models::DataTable {
                    name: "table".to_owned(),
                    cells: vec! {
                        crate::models::DataCell {
                            coord: crate::models::Coord { row: 1, col: 2 },
                            value: "Bart Simpson".to_owned(),
                        },
                    },
                },
            };
        let output =
            crate::models::TableUpdateResponse {
                version: "1.0.0".to_owned(),
                table: crate::models::ErrorTable {
                    name: "table".to_owned(),
                    cells: vec! {
                        crate::models::ErrorCell {
                            coord: crate::models::Coord { row: 1, col: 2 },
                            error: crate::models::Error { code: 10123, message: "nuclear hazard detected".to_owned() },
                        },
                    },
                }
            };

        let client = rocket::local::Client::new(crate::server()).unwrap();
        let payload = serde_json::to_string(&input).unwrap();
        let request = client
            .post("/api/table_update")
            .header(rocket::http::ContentType::JSON)
            .body(payload)
            .dispatch();
        let response = request.await;
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

}
