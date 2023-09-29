extern crate serde_json;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use utoipa::{ToSchema, IntoParams};
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Customer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub id_client: u32,
    pub nom: String,
    pub prenom: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr5: Option<String>,
    pub pays: String,
    pub code_postal: String,
    pub ville: String,
    pub tel: String,
    pub mobile: String,
    pub email: String,
    pub Code_CE: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_ce_decla: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optin_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optin_fid_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_naissance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Moment_joindre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Demarchage_Tel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Demarchage_mobile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statut: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optin_SMS: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Bloctel_tel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Bloctel_mobile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Source_origine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Mention_legale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_inscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicule_concurrent: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts_entreprise: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicule_Simple: Option<Value>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Customers {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub id_client: u32,
    pub nom: String,
    pub prenom: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr5: Option<String>,
    pub pays: String,
    pub code_postal: String,
    pub ville: String,
    pub tel: String,
    pub mobile: String,
    pub email: String,
    pub Code_CE: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicule_Simple: Option<Vec<Vehicle>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libelle_court_gamme: Option<String>,
}

impl Default for Customer {
    fn default() -> Self {
        Customer {
            id: None,
            id_client: 0,
            nom: "".to_string(),
            prenom: "".to_string(),
            adr2: None,
            adr4: None,
            adr5: None,
            pays: "".to_string(),
            code_postal: "".to_string(),
            ville: "".to_string(),
            tel: "".to_string(),
            mobile: "".to_string(),
            email: "".to_string(),
            Code_CE: "".to_string(),
            code_ce_decla: None,
            optin_email: None,
            optin_fid_email: None,
            date_naissance: None,
            Moment_joindre: None,
            Demarchage_Tel: None,
            Demarchage_mobile: None,
            statut: None,
            optin_SMS: None,
            Bloctel_tel: None,
            Bloctel_mobile: None,
            Source_origine: None,
            Mention_legale: None,
            inscription: None,
            pre_inscription: None,
            vehicule_concurrent: None,
            contacts_entreprise: None,
            vehicule_Simple: None,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct QueryOptions {
    /// The collection page number
    pub page: Option<usize>,
    /// The number of items per page
    pub item_per_page: Option<usize>,
    /// Filter using multiple criteria, idClient, client firstname, client lastname, Code CE or client vehicle vin
    pub search: Option<String>,
}
