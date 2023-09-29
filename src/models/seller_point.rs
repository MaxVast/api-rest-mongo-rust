extern crate serde_json;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use utoipa::{ToSchema, IntoParams};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SellerPoint {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Code_SitePrimaire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Code_SiteDeVente: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Concession: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Adr3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Adr4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Adr5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CodePostal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Ville: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Date_Ouvertue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Date_Fermeture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_monde: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_web: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GPS_Longitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GPS_Latitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codeCE: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SZM_SiteDeVente_datemodif: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_departement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_localite: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adresse_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capital_social: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_forme_juridique: Option<u32>,
}

impl Default for SellerPoint {
    fn default() -> Self {
        SellerPoint {
            id: None,
            Code_SitePrimaire: None,
            Code_SiteDeVente: None,
            Concession: None,
            Adr3: None,
            Adr4: None,
            Adr5: None,
            CodePostal: None,
            Ville: None,
            Telephone: None,
            Date_Ouvertue: None,
            Date_Fermeture: None,
            rcs: None,
            email: None,
            id_monde: None,
            site_web: None,
            GPS_Longitude: None,
            GPS_Latitude: None,
            codeCE: None,
            SZM_SiteDeVente_datemodif: None,
            code_departement: None,
            id_localite: None,
            adresse_ip: None,
            capital_social: None,
            id_forme_juridique: None,
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
