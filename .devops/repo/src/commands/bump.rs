use crate::utils::print_applications;
use std::process::Command;

pub(crate) fn run(apps: Vec<String>, root: String) {
    let mut success: Vec<String> = vec![];

    let toolchain = "fuel-toolchain.toml".to_string();
    let tmp_toolchain = "fuel-toolchain2.toml".to_string();

    for app in apps {
        println!("\nBumping {}", app);

        // TODO: safety
        let project = std::fs::canonicalize(format!("{}/{}/project", root, app)).unwrap();

        let _ = Command::new("mv")
            .env("IFS", "''")
            .args([
                format!("{}/{}", project.clone().display(), toolchain.clone()),
                format!("{}/{}", project.clone().display(), tmp_toolchain.clone()),
            ])
            .status();

        let _ = Command::new("cp")
            .env("IFS", "''")
            .args([
                format!("./{}", toolchain.clone()),
                format!("{}/{}", project.clone().display(), toolchain.clone()),
            ])
            .status();

        let result = Command::new("forc")
            .current_dir(project.clone())
            .arg("build")
            .status();

        match result {
            Ok(status) => {
                if status.success() {
                    success.push(app.clone());
                    let _ = Command::new("rm")
                        .current_dir(project.clone())
                        .arg(tmp_toolchain.clone())
                        .status();
                } else {
                    let _ = Command::new("mv")
                        .env("IFS", "''")
                        .current_dir(project.clone())
                        .args([
                            format!("{}/{}", project.clone().display(), tmp_toolchain.clone()),
                            format!("{}/{}", project.display(), toolchain.clone()),
                        ])
                        .status();
                }
            }
            Err(_) => {
                let _ = Command::new("mv")
                    .env("IFS", "''")
                    .current_dir(project.clone())
                    .args([
                        format!("{}/{}", project.clone().display(), tmp_toolchain.clone()),
                        format!("{}/{}", project.display(), toolchain.clone()),
                    ])
                    .status();
            }
        }
    }

    print_applications(success, "Bumped".to_string());
}
