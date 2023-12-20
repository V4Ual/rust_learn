
use serde_json::json;

pub fn success_response(data: serde_json::Value) -> serde_json::Value {
    json!({
        "success": true,
        "data": data,
    })
}