use axum::Extension;

use super::SharedData;

pub async fn middleware_message(Extension(sharedata) : Extension<SharedData>)-> String{
    sharedata.message
}