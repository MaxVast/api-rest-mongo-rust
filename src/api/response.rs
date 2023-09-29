use serde::Serialize;

use crate::models::customer::{Customer, Customers};
use crate::models::seller_point::SellerPoint;

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct CustomerData {
    pub customer: Customer,
}

#[derive(Serialize, Debug)]
pub struct SellerPointData {
    pub seller_point: SellerPoint,
}

#[derive(Serialize, Debug)]
pub struct SingleCustomerResponse {
    pub data: CustomerData,
}

#[derive(Serialize, Debug)]
pub struct CustomerListResponse {
    pub results: usize,
    pub max_page: usize,
    pub data: Vec<Customers>,
}

#[derive(Serialize, Debug)]
pub struct SellerPointListResponse {
    pub results: usize,
    pub max_page: usize,
    pub data: Vec<SellerPoint>,
}

#[derive(Serialize, Debug)]
pub struct SingleSellerPointResponse {
    pub data: SellerPointData,
}
