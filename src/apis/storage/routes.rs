use actix_web::{web};

use crate::apis::storage::reqs as reqs;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/{tail:.*}")
                .route("", web::get()
                    .to(reqs::get)
                )
                .route("", web::post()
                    .to(reqs::post)
                )
                .route("", web::put()
                    .to(reqs::put)
                )
                .route("", web::delete()
                    .to(reqs::delete)
                )
                .route("", web::head()
                    .to(reqs::head)
                )
        );
}
