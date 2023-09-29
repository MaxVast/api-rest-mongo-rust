use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, self, Bson, Document},
    Client, Collection,
};
use mongodb::options::AggregateOptions;
use crate::models::customer::{Customer, Customers};
use crate::models::seller_point::SellerPoint;

pub struct MongoRepo {
    col: Collection<Customer>,
    col2: Collection<SellerPoint>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable URI MONGO"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db_name = match env::var("MONGODBDB") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let db_collection = match env::var("MONGOCOLLECTION") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let db = client.database(&db_name);
        let col: Collection<Customer> = db.collection(&db_collection);
        let col2: Collection<SellerPoint> = db.collection("sellerPoint");

        println!("✅ Database connected successfully");
        MongoRepo { col, col2 }
    }

    pub async fn get_customer(&self, id: &u32) -> Result<Customer, Error> {
        let filter = doc! {"id_client": id};
        let customer_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting customer's detail");
        Ok(customer_detail.unwrap())
    }

    //OLD REQUEST MONGO DB
    /*pub async fn get_all_customer(&self) -> Result<Vec<Customer>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut customers: Vec<Customer> = Vec::new();
        while let Some(customer) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            customers.push(customer)
        }
        Ok(customers)
    }*/

    pub async fn get_all_customer_by_filter(&self, query: &String) -> Result<Vec<Customers>, Error> {
        let value = query;
        let pipeline = vec![
            doc! {
                "$match": {
                    "$or": [
                        {"id_client": Bson::Int32(value.parse::<i32>().unwrap_or(0))},
                        {"nom": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                        {"prenom": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                        {"Code_CE": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                        {"vehicule_Simple.code_vin": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                        {"vehicule_Simple.libelle_court_gamme": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })}
                    ]
                }
            }
        ];
        let aggregate_options = AggregateOptions::builder().build();
        let mut customers: Vec<Customers> = Vec::new();
        let mut cursors = self
            .col
            .aggregate(pipeline, aggregate_options)
            .await
            .expect("Error getting list of customers");
        while let Ok(Some(document)) = cursors.try_next().await {
            match bson::from_document::<Customers>(document) {
                Ok(customer) => customers.push(customer),
                Err(e) => {
                    eprintln!("Error mapping through cursor on Customer : {:?}", e);
                }
            }
        }

        Ok(customers)
    }

    pub async fn get_all_seller_point_by_filter(&self, query: &String) -> Result<Vec<SellerPoint>, Error> {
        let value = query;
        let pipeline = vec![
            doc! {
                "$match": {
                    "$or": [
                        {"Code_SitePrimaire": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                        {"Code_SiteDeVente": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                        {"codeCE": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                        {"code_departement": Bson::RegularExpression(bson::Regex { pattern: value.to_string(), options: "i".to_string() })},
                    ]
                }
            }
        ];
        let aggregate_options = AggregateOptions::builder().build();
        let mut seller_points: Vec<SellerPoint> = Vec::new();
        let mut cursors = self
            .col2
            .aggregate(pipeline, aggregate_options)
            .await
            .expect("Error getting list of customers");
        while let Ok(Some(document)) = cursors.try_next().await {
            match bson::from_document::<SellerPoint>(document) {
                Ok(seller_point) => seller_points.push(seller_point),
                Err(e) => {
                    eprintln!("Error mapping through cursor on Customer : {:?}", e);
                }
            }
        }

        Ok(seller_points)
    }

    pub async fn get_seller_point(&self, code_ce: &String) -> Result<SellerPoint, Error> {
        let filter = doc! {"Code_SiteDeVente": code_ce.to_string()};
        let seller_point_detail = self
            .col2
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting customer's detail");
        Ok(seller_point_detail.unwrap())
    }
}