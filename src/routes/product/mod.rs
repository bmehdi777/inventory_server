use serde::{Deserialize, Serialize};

pub mod delete;
pub mod get;
pub mod post;
pub mod put;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub image: Option<String>,
    pub quantity: Option<u64>,
    pub category: Option<Vec<String>>,
    pub barcode: Option<String>,
}

impl From<OpenFoodFactProduct> for Product {
    fn from(product: OpenFoodFactProduct) -> Self {
        Product {
            name: product.product.name,
            image: Some(product.product.image),
            quantity: None,
            category: None,
            barcode: Some(product.id),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenFoodFactProduct {
    #[serde(alias = "code")]
    pub id: String,

    pub product: OpenFoodFactProductDetail,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenFoodFactProductDetail {
    #[serde(alias = "product_name_fr")]
    pub name: String,

    #[serde(alias = "image_url")]
    pub image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductQuery {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductUpdate {
    pub current_name: String,
    pub new_quantity: f64,
    pub new_categories: Vec<String>,
}
