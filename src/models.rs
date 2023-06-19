use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProjectRepo {
    pub slug: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectReposResponse {
    #[serde(alias = "isLastPage")]
    pub is_last_page: bool,
    pub values: Vec<ProjectRepo>,
}

#[derive(Deserialize, Debug)]
pub struct RepoFilesResponse {
    #[serde(alias = "isLastPage")]
    pub is_last_page: bool,
    pub values: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RepoFileContentResponse {
    pub lines: Vec<RepoFileContentLine>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RepoFileContentLine {
    pub text: String,
}
#[derive(Debug)]
pub struct RepoFile {
    pub name: String,
    pub content: RepoFileContentResponse,
}

impl TryFrom<RepoFileContentLine> for PackageReference {
    type Error = ();

    fn try_from(value: RepoFileContentLine) -> Result<Self, Self::Error> {
        if !value.text.contains("<PackageReference") {
            return Err(());
        }

        let parts: Vec<&str> = value.text.split(' ').collect();

        let name_part = parts.iter().find(|s| s.starts_with("Include")).copied();
        let version_part = parts.iter().find(|s| s.starts_with("Version")).copied();

        if name_part.is_none() || version_part.is_none() {
            return Err(());
        }

        let name_part = name_part.unwrap();
        let version_part = version_part.unwrap();

        let package_name = name_part.replace("Include", "").replace(['\"', '='], "");

        let version_str = version_part.replace("Version", "").replace(['\"', '='], "");

        let version_parts: Vec<u32> = version_str
            .splitn(3, '.')
            .map(|part| part.parse::<u32>().unwrap_or(0))
            .collect();

        let major = version_parts.first().copied();
        let minor = version_parts.get(1).copied();
        let patch = version_parts.get(2).copied();

        let package_version = PackageVersion::new(major, minor, patch);

        Ok(PackageReference {
            package_name,
            package_version,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PackageReference {
    pub package_name: String,
    pub package_version: PackageVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct PackageVersion {
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
}

impl PackageVersion {
    pub fn new(major: Option<u32>, minor: Option<u32>, patch: Option<u32>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl ToString for PackageVersion {
    fn to_string(&self) -> String {
        let major_str = self.major.map_or("?".to_owned(), |n| n.to_string());
        let minor_str = self.minor.map_or("?".to_owned(), |n| n.to_string());
        let patch_str = self.patch.map_or("?".to_owned(), |n| n.to_string());

        format!("{}.{}.{}", major_str, minor_str, patch_str)
    }
}

#[derive(Debug, Clone)]
pub struct RepoPackageReference {
    pub repo_name: String,
    pub package_reference: PackageReference,
    pub csproj_file_name: String,
}

impl RepoPackageReference {
    pub fn new(
        repo_name: String,
        csproj_file_name: String,
        package_reference: PackageReference,
    ) -> Self {
        Self {
            repo_name,
            package_reference,
            csproj_file_name,
        }
    }
}
