pub mod health_check;
pub mod product;
pub mod user;
pub mod login;
pub mod search;

const USER_TABLENAME: &'static str = "users";
const PRODUCT_TABLENAME: &'static str = "product";

const API_BARCODE: &'static str = "https://world.openfoodfacts.org/api/v0/product/";
