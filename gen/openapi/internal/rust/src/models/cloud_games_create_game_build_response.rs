/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGamesCreateGameBuildResponse {
    #[serde(rename = "build_id")]
    pub build_id: uuid::Uuid,
    #[serde(rename = "image_presigned_request")]
    pub image_presigned_request: Box<crate::models::UploadPresignedRequest>,
    #[serde(rename = "image_presigned_requests")]
    pub image_presigned_requests: Vec<crate::models::UploadPresignedRequest>,
    #[serde(rename = "upload_id")]
    pub upload_id: uuid::Uuid,
}

impl CloudGamesCreateGameBuildResponse {
    pub fn new(build_id: uuid::Uuid, image_presigned_request: crate::models::UploadPresignedRequest, image_presigned_requests: Vec<crate::models::UploadPresignedRequest>, upload_id: uuid::Uuid) -> CloudGamesCreateGameBuildResponse {
        CloudGamesCreateGameBuildResponse {
            build_id,
            image_presigned_request: Box::new(image_presigned_request),
            image_presigned_requests,
            upload_id,
        }
    }
}


