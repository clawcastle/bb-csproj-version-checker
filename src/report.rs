use itertools::Itertools;

use crate::models::RepoPackageReference;

#[derive(Default)]
pub struct PackageVersionReport {
    pub repo_package_references: Vec<RepoPackageReference>,
}

impl ToString for PackageVersionReport {
    fn to_string(&self) -> String {
        let mut s = String::new();

        let grouped_by_repo = self
            .repo_package_references
            .iter()
            .group_by(|r| r.repo_name.clone());

        for (repo_slug, group) in grouped_by_repo.into_iter() {
            let repo_title_line = format!("## {}\n", &repo_slug.to_uppercase());
            s += &repo_title_line;
            s += "\n";

            let sorted: Vec<RepoPackageReference> = group
                .sorted_by(|a, b| {
                    Ord::cmp(
                        &a.package_reference.package_version,
                        &b.package_reference.package_version,
                    )
                })
                .rev()
                .cloned()
                .collect();

            for p in sorted.iter() {
                let project_name = p
                    .csproj_file_name
                    .split('/')
                    .last()
                    .unwrap()
                    .replace(".csproj", "");

                s += &format!(
                    "Repository: {}. Project name: {}. {} package version: {}.\n",
                    p.repo_name,
                    project_name,
                    p.package_reference.package_name,
                    p.package_reference.package_version.to_string()
                );
            }
        }

        s += "\n\n";
        s
    }
}
