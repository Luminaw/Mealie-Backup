use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct AllBackups {
    #[serde(rename = "imports")]
    pub backups: Vec<Backup>,
    pub templates: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Backup {
    pub name: String,
    pub date: String,
    pub size: String,
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse {
    pub message: String,
    pub error: bool,
}

#[derive(Debug, Deserialize)]
pub struct FileTokenResponse {
    #[serde(rename = "fileToken")]
    pub file_token: String,
}

#[derive(Debug, Deserialize)]
pub struct HTTPValidationError {
    pub detail: Vec<ValidationError>,
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<String>,
    pub msg: String,
    pub r#type: String,
}

pub struct BackupService {
    base_url: String,
    token: String,
    client: Client,
}

impl BackupService {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            base_url,
            token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_all_backups(&self, accept_language: Option<String>) -> Result<AllBackups, Box<dyn Error>> {
        let url = format!("{}/api/admin/backups", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(lang) = accept_language {
            headers.insert("Accept-Language", lang.parse()?);
        }
        headers.insert("Authorization", format!("Bearer {}", self.token).parse()?);
        
        let response = self.client.get(&url).headers(headers).send().await?;
        let all_backups = response.json::<AllBackups>().await?;
        Ok(all_backups)
    }

    pub async fn create_backup(&self, accept_language: Option<String>) -> Result<SuccessResponse, Box<dyn Error>> {
        let url = format!("{}/api/admin/backups", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(lang) = accept_language {
            headers.insert("Accept-Language", lang.parse()?);
        }
        headers.insert("Authorization", format!("Bearer {}", self.token).parse()?);
        
        let response = self.client.post(&url).headers(headers).send().await?;
        let success_response = response.json::<SuccessResponse>().await?;
        Ok(success_response)
    }

    pub async fn get_backup(&self, file_name: &String, accept_language: Option<String>) -> Result<FileTokenResponse, Box<dyn Error>> {
        let url = format!("{}/api/admin/backups/{}", self.base_url, file_name);
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(lang) = accept_language {
            headers.insert("Accept-Language", lang.parse()?);
        }
        headers.insert("Authorization", format!("Bearer {}", self.token).parse()?);
        
        let response = self.client.get(&url).headers(headers).send().await?;
        let file_token_response = response.json::<FileTokenResponse>().await?;

        Ok(file_token_response)
    }

    pub async fn delete_backup(&self, file_name: String, accept_language: Option<String>) -> Result<SuccessResponse, Box<dyn Error>> {
        let url = format!("{}/api/admin/backups/{}", self.base_url, file_name);
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(lang) = accept_language {
            headers.insert("Accept-Language", lang.parse()?);
        }
        headers.insert("Authorization", format!("Bearer {}", self.token).parse()?);
        
        let response = self.client.delete(&url).headers(headers).send().await?;
        let success_response = response.json::<SuccessResponse>().await?;
        Ok(success_response)
    }

    pub async fn download_backup(&self, file_token: FileTokenResponse) -> Result<Vec<u8>, Box<dyn Error>> {
        let url = format!("{}/api/utils/download", self.base_url);
        let response = self.client.get(&url)
            .query(&[("token", &file_token.file_token)])
            .send().await?;
        
        if response.status().is_success() {
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            let error = response.json::<HTTPValidationError>().await?;
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", error))))
        }
    }
}