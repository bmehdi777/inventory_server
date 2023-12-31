use crate::{
    routes::{product::Product, PRODUCT_TABLENAME},
    startup::AppStateRC,
    utils::AppError,
};
use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::doc;

use super::ProductUpdate;

pub async fn update(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<ProductUpdate>,
) -> Result<StatusCode, AppError> {
    app_state
        .database
        .collection::<Product>(PRODUCT_TABLENAME)
        .update_one(
            doc! {"name": payload.current_name},
            doc! {"category": payload.new_categories, "quantity": payload.new_quantity },
            None,
        )
        .await?;
    Ok(StatusCode::OK)
}
