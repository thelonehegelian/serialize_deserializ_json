use serde_json::{json, Error, Value};

fn main() -> Result<(), Error> {
    let json_data = r#"{
        "id": 1, 
        "title": "iPhone 9",
        "description": "An apple mobile which is nothing like apple",
        "price": 549,
        "discountPercentage": 12.96,
        "rating": 4.69,
        "stock": 94,
        "brand": "Apple",
        "category": "smartphones",
        "thumbnail": "https://i.dummyjson.com/data/products/1/thumbnail.jpg"
      }"#;

    let parsed_json: Value = serde_json::from_str(json_data)?;

    println!("{:?}", parsed_json);

    // generates a serde json value from a string
    let _ = json!({   "id": 1, 
    "title": "iPhone 9",
    "description": "An apple mobile which is nothing like apple",
    "price": 549,
    "discountPercentage": 12.96,
    "rating": 4.69,
    "stock": 94,
    "brand": "Apple",
    "category": "smartphones",
    "thumbnail": "https://i.dummyjson.com/data/products/1/thumbnail.jpg"});

    Ok(())
}
