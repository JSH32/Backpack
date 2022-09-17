use actix_web::{web, Scope};

pub fn get_routes() -> Scope {
    web::scope("/file")
}

// Get a paginated list of files
// - Minimum required role: `admin`
// - Allow unverified users: `true`
// - Application token allowed: `false`
// #[utoipa::path(
//     context_path = "/api/file/list",
//     tag = "admin",
//     responses(
//         (status = 200, body = FilePage),
//         (status = 400, body = MessageResponse, description = "Invalid page number"),
//         (status = 404, body = MessageResponse, description = "Page not found")
//     ),
//     params(
//         ("page_number" = usize, Path, description = "Page to get"),
//         FileQuery
//     ),
//     security(("apiKey" = [])),
// )]
// #[get("/list/{page_number}")]
// async fn list(
//     state: web::Data<State>,
//     page_number: web::Path<usize>,
//     query: web::Query<FileQuery>,
//     _user: Auth<auth_role::Admin>,
// ) -> Response<impl Responder> {
//     let mut condition = Condition::all();
//     if let Some(user_id) = &query.user {
//         condition = condition.add(files::Column::Uploader.eq(user_id.clone()));
//     }

//     if let Some(search) = &query.search {
//         condition = condition.add(files::Column::Name.like(&format!("%{}%", search.clone())));
//     }

//     let paginator = files::Entity::find()
//         .filter(condition)
//         .order_by_desc(files::Column::Uploaded)
//         .paginate(&state.database, 25);

//     let pages = paginator.num_pages().await?;
//     if let Some(err) = validate_paginate(*page_number, pages) {
//         return Ok(err.http_response());
//     }

//     let storage_url = PathBuf::from(state.storage_url.clone());
//     Ok(HttpResponse::Ok().json(Page {
//         page: *page_number,
//         pages,
//         items: paginator
//             .fetch_page(*page_number - 1)
//             .await?
//             .iter()
//             .map(|model| {
//                 let mut file_data = FileData::from(model.to_owned());
//                 file_data.set_url(storage_url.clone());
//                 file_data.set_thumbnail_url(storage_url.clone());
//                 file_data
//             })
//             .collect(),
//     }))
// }
