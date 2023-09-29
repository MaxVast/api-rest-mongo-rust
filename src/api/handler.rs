use crate::{
    models::customer::{Customer, Customers, QueryOptions},
    models::seller_point::{SellerPoint},
    api::response::{GenericResponse, SingleCustomerResponse, CustomerData,
    CustomerListResponse, SellerPointListResponse, SellerPointData, SingleSellerPointResponse},
};
use actix_web::{get, web, web::{Data, Path}, HttpResponse, Responder};

use crate::db::db::MongoRepo;
use utoipa;

#[get("/health-check")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple API with Rust, Actix Web and MongoDB";

    let response_json = &GenericResponse {
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[utoipa::path(
    get,
    path = "/api/customers",
    responses(
        (status = 200, description = "List current customer items")
    ),
    params(QueryOptions)
)]
#[get("/customers")]
pub async fn customers_list(db: Data<MongoRepo>, opts: web::Query<QueryOptions>) -> impl Responder {
    let customer_search: String = opts.search.as_ref().map_or_else(|| String::default(), |s| s.clone());
    let customer = db.get_all_customer_by_filter(&customer_search).await;
    let customers: Vec<Customers> = match customer {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Erreur : {:?}", error);
            Vec::new()
        }
    };

    let total_items: usize = customers.len();
    let limit: usize = opts.item_per_page.unwrap_or(20);
    let page:usize = (opts.page.unwrap_or(1) - 1) * limit;
    let max_page = (total_items as f64 / limit as f64).ceil() as usize;
    let customers: Vec<Customers>  = customers.into_iter().skip(page).take(limit).collect();
    let json_response = CustomerListResponse {
        results: total_items,
        max_page: max_page,
        data: customers
    };
    HttpResponse::Ok().json(json_response)
}

#[utoipa::path(
    get,
    path = "/api/customer/{id_client}",
    responses(
        (status = 200, description = "Customer found from storage"),
        (status = 404, description = "Customer not found by id_client")
    ),
    params(
        ("id_client", description = "Unique storage id of Customer")
    )
)]
#[get("/customer/{id_client}")]
pub async fn get_customer(db: Data<MongoRepo>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    if id.is_empty() {
        let error_response = GenericResponse {
            message: format!("No id_client"),
        };
        return HttpResponse::NotFound().json(error_response);
    }
    let id_client: u32 = id.parse().unwrap();
    let customer_data = db.get_customer(&id_client).await;
    let customer: Customer = match customer_data {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Erreur : {:?}", error);
            Customer::default()
        }
    };
    /*if customer.is_none() {
        let error_response = GenericResponse {
            message: format!("Customer with id_client: {} not found", id_client),
        };
        return HttpResponse::NotFound().json(error_response);
    }*/

    let json_response = SingleCustomerResponse {
        data: CustomerData { customer },
    };

    HttpResponse::Ok().json(json_response)
}

#[utoipa::path(
    get,
    path = "/api/seller-points",
    responses(
        (status = 200, description = "List current seller points items")
    ),
    params(QueryOptions)
)]
#[get("/seller-points")]
pub async fn seller_points_list(db: Data<MongoRepo>, opts: web::Query<QueryOptions>) -> impl Responder {
    let seller_point_search: String = opts.search.as_ref().map_or_else(|| String::default(), |s| s.clone());
    let seller_point = db.get_all_seller_point_by_filter(&seller_point_search).await;
    let seller_points: Vec<SellerPoint> = match seller_point {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Erreur : {:?}", error);
            Vec::new()
        }
    };

    let total_items: usize = seller_points.len();
    let limit: usize = opts.item_per_page.unwrap_or(20);
    let page: usize = (opts.page.unwrap_or(1) - 1) * limit;
    let max_page = (total_items as f64 / limit as f64).ceil() as usize;
    let seller_points: Vec<SellerPoint>  = seller_points.into_iter().skip(page).take(limit).collect();
    let json_response = SellerPointListResponse {
        results: total_items,
        max_page: max_page,
        data: seller_points
    };
    HttpResponse::Ok().json(json_response)
}

#[utoipa::path(
    get,
    path = "/api/seller-point/{code_ce}",
    responses(
        (status = 200, description = "Seller Point  found from storage"),
        (status = 404, description = "Seller Point not found by code_ce")
    ),
    params(
        ("code_ce", description = "Unique storage code ce of Seller Point")
    )
)]
#[get("/seller-point/{code_ce}")]
pub async fn get_seller_point(db: Data<MongoRepo>, path: Path<String>) -> impl Responder {
    let code_ce = path.into_inner();
    if code_ce.is_empty() {
        let error_response = GenericResponse {
            message: format!("No code ce"),
        };
        return HttpResponse::NotFound().json(error_response);
    }
    let seller_point_data = db.get_seller_point(&code_ce.to_string()).await;
    let seller_point: SellerPoint = match seller_point_data {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Erreur : {:?}", error);
            SellerPoint::default()
        }
    };
    /*if customer.is_none() {
        let error_response = GenericResponse {
            message: format!("Customer with id_client: {} not found", id_client),
        };
        return HttpResponse::NotFound().json(error_response);
    }*/

    let json_response = SingleSellerPointResponse {
        data: SellerPointData { seller_point },
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(customers_list)
        .service(get_customer)
        .service(seller_points_list)
        .service(get_seller_point);

    conf.service(scope);
}