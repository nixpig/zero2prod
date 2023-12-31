use serde::Deserialize;

use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    form: Form<FormData>,
    connection: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        "insert into subscriptions (name, email) values ($1, $2)",
        form.name,
        form.email
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
