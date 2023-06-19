use crate::models::{
    ProjectRepo, ProjectReposResponse, RepoFileContentResponse, RepoFilesResponse,
};

pub struct BitbucketClient<'a> {
    base_url: &'a str,
    access_token: &'a str,
    client: reqwest::Client,
}

impl<'a> BitbucketClient<'a> {
    pub fn new(base_url: &'a str, access_token: &'a str) -> Self {
        Self {
            base_url,
            access_token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_project_repos(
        &self,
        project_key: &str,
    ) -> Result<Vec<ProjectRepo>, anyhow::Error> {
        let mut project_repos: Vec<ProjectRepo> = Vec::new();
        let mut start_page = 0;

        let get_projects_url = format!(
            "https://{}/rest/api/latest/projects/{}/repos?limit=50",
            &self.base_url, &project_key
        );

        loop {
            let url = format!("{}&start={}", &get_projects_url, start_page);

            let response: ProjectReposResponse = self
                .client
                .get(&url)
                .bearer_auth(self.access_token)
                .header("Accept", "application/json")
                .send()
                .await?
                .json()
                .await?;

            project_repos.extend(response.values);

            if response.is_last_page {
                break;
            }

            start_page += 1;
        }

        Ok(project_repos)
    }

    pub async fn get_paths_of_files_in_repo(
        &self,
        project_key: &str,
        repo_slug: &str,
        file_extension: Option<&'a str>,
    ) -> Result<Vec<String>, anyhow::Error> {
        let get_repo_files_url = format!(
            "https://{}/rest/api/latest/projects/{}/repos/{}/files/?limit=1000",
            &self.base_url, &project_key, &repo_slug
        );
        let mut start_page = 0;

        let mut file_paths: Vec<String> = Vec::new();

        loop {
            let url = format!("{}&start={}", &get_repo_files_url, start_page);

            let response: RepoFilesResponse = self
                .client
                .get(&url)
                .bearer_auth(self.access_token)
                .header("Accept", "application/json")
                .send()
                .await?
                .json()
                .await?;

            for file in response.values {
                if let Some(extension) = file_extension {
                    if !file.ends_with(extension) {
                        continue;
                    }
                }
                file_paths.push(file.clone());
            }

            if response.is_last_page {
                break;
            }

            start_page += 1;
        }

        Ok(file_paths)
    }

    pub async fn get_repo_file_content(
        &self,
        project_key: &str,
        repo_slug: &str,
        file_path: &str,
    ) -> Result<RepoFileContentResponse, anyhow::Error> {
        let get_file_content_url = format!(
            "https://{}/rest/api/latest/projects/{}/repos/{}/browse/{}",
            &self.base_url, &project_key, &repo_slug, &file_path
        );

        let response: RepoFileContentResponse = self
            .client
            .get(&get_file_content_url)
            .bearer_auth(self.access_token)
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
