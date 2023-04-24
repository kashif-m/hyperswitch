use actix_web::{
    body::{BoxBody, MessageBody},
    HttpResponse, Responder,
};
use router_env::{instrument, tracing, Flow};

use crate::{
    connection,
    core::{
        errors::{self, CustomResult, RouterResponse, RouterResult},
        payment_methods::vault,
    },
    db::StorageInterface,
    logger, pii,
    routes::AppState,
    scheduler::utils as pt_utils,
    services,
    types::{
        self, api,
        storage::{self, enums as storage_enums},
    },
    utils::{Encode, OptionExt, ValueExt},
};

/// Payouts - Create
#[utoipa::path(
    post,
    path = "/payouts",
    request_body=PayoutsRequest,
    responses(
        (status = 200, description = "Payout created", body = PayoutsResponse),
        (status = 400, description = "Missing Mandatory fields")
    ),
    tag = "Payouts",
    operation_id = "Create a Payout",
    security(("api_key" = []))
)]
#[instrument(skip_all, fields(flow = ?Flow::PayoutsCreate))]
// #[post("")]
pub async fn payouts_create(
    state: web::Data<app::AppState>,
    req: actix_web::HttpRequest,
    json_payload: web::Json<payment_types::PayoutsRequest>,
) -> impl Responder {
    let flow = Flow::PayoutsCreate;
    let payload = json_payload.into_inner();

    api::server_wrap(
        flow,
        state.get_ref(),
        &req,
        payload,
        |state, merchant_account, req| {
            authorize_verify_select(
                payouts::PayoutCreate,
                state,
                merchant_account,
                req,
                api::AuthFlow::Merchant,
            )
        },
        &auth::ApiKeyAuth,
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::PayoutsRetrieve))]
// #[get("/retrieve")]
pub async fn payouts_retrieve() -> impl Responder {
    let _flow = Flow::PayoutsRetrieve;
    http_response("retrieve")
}

#[instrument(skip_all, fields(flow = ?Flow::PayoutsUpdate))]
// #[post("/update")]
pub async fn payouts_update() -> impl Responder {
    let _flow = Flow::PayoutsUpdate;
    http_response("update")
}

#[instrument(skip_all, fields(flow = ?Flow::PayoutsReverse))]
// #[post("/reverse")]
pub async fn payouts_reverse() -> impl Responder {
    let _flow = Flow::PayoutsReverse;
    http_response("reverse")
}

#[instrument(skip_all, fields(flow = ?Flow::PayoutsCancel))]
// #[post("/cancel")]
pub async fn payouts_cancel() -> impl Responder {
    let _flow = Flow::PayoutsCancel;
    http_response("cancel")
}

#[instrument(skip_all, fields(flow = ?Flow::PayoutsAccounts))]
// #[get("/accounts")]
pub async fn payouts_accounts() -> impl Responder {
    let _flow = Flow::PayoutsAccounts;
    http_response("accounts")
}

fn http_response<T: MessageBody + 'static>(response: T) -> HttpResponse<BoxBody> {
    HttpResponse::Ok().body(response)
}
