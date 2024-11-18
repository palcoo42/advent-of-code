use std::path::PathBuf;

/// Name of environment variable which stores path to the Cargo.toml, i.e. project root
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

/// Name of the subdirectory inside of the project root where resources are stored
const RESOURCES_SUB_DIR: &str = "resources";

/// Name of the main directory where resources for integration tests are stored
const INTEGRATION_TESTS_SUB_DIR: &str = "tests";

/// Provides information about current project within workspace
#[derive(Default)]
pub struct Project {
    // Path to the project root directory
    path: PathBuf,
}

impl Project {
    ///  Creates a new instance which manages paths to the project and its subdirectories
    ///
    /// # Returns
    ///
    /// New instance of the Project
    ///
    /// # Panics
    ///
    /// This function will panic when environment variable _CARGO_MANIFEST_DIR_ is not found.
    pub fn new() -> Self {
        let toml_path = std::env::var(CARGO_MANIFEST_DIR)
            .unwrap_or_else(|_| panic!("Environment variable '{}' not found", CARGO_MANIFEST_DIR));

        Self {
            path: PathBuf::from(toml_path),
        }
    }

    /// Get the path to the project root directory
    ///
    /// # Returns
    ///
    /// Path to the project root directory
    ///
    /// # Examples
    ///
    /// Consider following project structure and assume we are in the _advent-of-code_ project.
    ///
    /// ```text
    ///
    ///    advent-of-code
    ///    ├── resources
    ///    │   ├── png
    ///    │   │   ├── foo.png
    ///    │   │   └── bar.png
    ///    │   └── README.md
    ///    ├── src
    ///    │   ├── main.rs
    ///    │   └── lib.rs
    ///    ├── tests
    ///    │   ├── resources
    ///    │   │   ├── input.txt
    ///    │   │   └── output.txt
    ///    │   ├── test_me.rs
    ///    │   └── mod.rs
    ///    ├── Cargo.toml
    ///    └── README.md
    /// ```
    ///
    /// ```
    /// use crate::advent_of_code::env::project::Project;
    ///
    /// let project = Project::new();
    /// let path = project.root_path();
    ///
    /// assert!(path.ends_with("advent-of-code"));
    ///
    /// ```
    pub fn root_path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the path to the project directory which is stored under specified sub-directories structure.
    /// This is a generic method which provides a path to any directory under the project.
    ///
    /// # Arguments
    ///
    /// _sub_dirs_ - Array of sub directories which will be added to the project root directory
    ///
    /// # Returns
    ///
    /// Path to the project sub-directory
    ///
    /// # Examples
    ///
    /// Consider following project structure and assume we are in the _advent-of-code_ project.
    ///
    /// ```text
    ///
    ///    advent-of-code
    ///    ├── resources
    ///    │   ├── png
    ///    │   │   ├── foo.png
    ///    │   │   └── bar.png
    ///    │   └── README.md
    ///    ├── src
    ///    │   ├── main.rs
    ///    │   └── lib.rs
    ///    ├── tests
    ///    │   ├── resources
    ///    │   │   ├── input.txt
    ///    │   │   └── output.txt
    ///    │   ├── test_me.rs
    ///    │   └── mod.rs
    ///    ├── Cargo.toml
    ///    └── README.md
    /// ```
    ///
    /// ```
    /// use crate::advent_of_code::env::project::Project;
    ///
    /// let project = Project::new();
    /// let path = project.project_path(&["resources", "png"]);
    ///
    /// assert!(path.ends_with("advent-of-code/resources/png"));
    ///
    /// ```
    pub fn project_path(&self, sub_dirs: &[&str]) -> PathBuf {
        let mut path = self.path.clone();

        for sub_dir in sub_dirs {
            path = path.join(sub_dir);
        }

        path
    }

    /// Get the path to the project file which is stored under specified sub-directories structure.
    /// This is a generic method which provide path to any file under the project.
    ///
    /// # Arguments
    ///
    /// _sub_dirs_ - Array of sub directories which will be added to the project root directory
    /// _file_name_ - Name of the file at the end of sub-directories
    ///
    /// # Returns
    ///
    /// Path to the project file inside sub-directories
    ///
    /// # Examples
    ///
    /// Consider following project structure and assume we are in the _advent-of-code_ project.
    ///
    /// ```text
    ///
    ///    advent-of-code
    ///    ├── resources
    ///    │   ├── png
    ///    │   │   ├── foo.png
    ///    │   │   └── bar.png
    ///    │   └── README.md
    ///    ├── src
    ///    │   ├── main.rs
    ///    │   └── lib.rs
    ///    ├── tests
    ///    │   ├── resources
    ///    │   │   ├── input.txt
    ///    │   │   └── output.txt
    ///    │   ├── test_me.rs
    ///    │   └── mod.rs
    ///    ├── Cargo.toml
    ///    └── README.md
    /// ```
    ///
    /// ```
    /// use crate::advent_of_code::env::project::Project;
    ///
    /// let project = Project::new();
    /// let path = project.project_file(&["resources", "png"], "foo.png");
    ///
    /// assert!(path.ends_with("advent-of-code/resources/png/foo.png"));
    ///
    /// ```
    pub fn project_file(&self, sub_dirs: &[&str], file_name: &str) -> PathBuf {
        self.project_path(sub_dirs).join(file_name)
    }

    /// Get the path to the project resource which is stored under _resources_ directory.
    /// This is a specialized method to ease retrieval of project resources.
    ///
    /// # Arguments
    ///
    /// _file_name_ - Name of the file under _resources_ sub-directory
    ///
    /// # Returns
    ///
    /// Path to the project file inside _resources_ sub-directory
    ///
    /// # Examples
    ///
    /// Consider following project structure and assume we are in the _advent-of-code_ project.
    ///
    /// ```text
    ///
    ///    advent-of-code
    ///    ├── resources
    ///    │   ├── png
    ///    │   │   ├── foo.png
    ///    │   │   └── bar.png
    ///    │   └── README.md
    ///    ├── src
    ///    │   ├── main.rs
    ///    │   └── lib.rs
    ///    ├── tests
    ///    │   ├── resources
    ///    │   │   ├── input.txt
    ///    │   │   └── output.txt
    ///    │   ├── test_me.rs
    ///    │   └── mod.rs
    ///    ├── Cargo.toml
    ///    └── README.md
    /// ```
    ///
    /// ```
    /// use crate::advent_of_code::env::project::Project;
    ///
    /// let project = Project::new();
    /// let path = project.resource_file("README.md");
    ///
    /// assert!(path.ends_with("advent-of-code/resources/README.md"));
    ///
    /// ```
    pub fn resource_file(&self, file_name: &str) -> PathBuf {
        self.project_file(&[RESOURCES_SUB_DIR], file_name)
    }

    /// Get the path to the project resource which is stored under _tests/resources_ directory.
    /// This is a specialized method to ease retrieval of project resources.
    ///
    /// # Arguments
    ///
    /// _file_name_ - Name of the file under _tests/resources_ sub-directory
    ///
    /// # Returns
    ///
    /// Path to the project file inside _tests/resources_ sub-directory
    ///
    /// # Examples
    ///
    /// Consider following project structure and assume we are in the _advent-of-code_ project.
    ///
    /// ```text
    ///
    ///    advent-of-code
    ///    ├── resources
    ///    │   ├── png
    ///    │   │   ├── foo.png
    ///    │   │   └── bar.png
    ///    │   └── README.md
    ///    ├── src
    ///    │   ├── main.rs
    ///    │   └── lib.rs
    ///    ├── tests
    ///    │   ├── resources
    ///    │   │   ├── input.txt
    ///    │   │   └── output.txt
    ///    │   ├── test_me.rs
    ///    │   └── mod.rs
    ///    ├── Cargo.toml
    ///    └── README.md
    /// ```
    ///
    /// ```
    /// use crate::advent_of_code::env::project::Project;
    ///
    /// let project = Project::new();
    /// let path = project.resource_test_file("input.txt");
    ///
    /// assert!(path.ends_with("advent-of-code/tests/resources/input.txt"));
    ///
    /// ```
    pub fn resource_test_file(&self, file_name: &str) -> PathBuf {
        self.project_file(&[INTEGRATION_TESTS_SUB_DIR, RESOURCES_SUB_DIR], file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let project = Project::new();

        assert!(project.root_path().ends_with("advent-of-code"));
    }

    #[test]
    fn test_project_path() {
        let project = Project::new();
        let path = project.project_path(&["tests", "resources"]);

        assert!(
            path.ends_with("advent-of-code/tests/resources"),
            "path: {:?}",
            path
        );
    }

    #[test]
    fn test_project_resource() {
        let project = Project::new();
        let path = project.resource_file("input.txt");

        assert!(
            path.ends_with("advent-of-code/resources/input.txt"),
            "path: {:?}",
            path
        );
    }

    #[test]
    fn test_project_resource_test() {
        let project = Project::new();
        let path = project.resource_test_file("input.txt");

        assert!(
            path.ends_with("advent-of-code/tests/resources/input.txt"),
            "path: {:?}",
            path
        );
    }

    #[test]
    fn test_project_file() {
        let project = Project::new();
        let path = project.project_file(&["tests", "resources"], "input.txt");

        assert!(
            path.ends_with("advent-of-code/tests/resources/input.txt"),
            "path: {:?}",
            path
        );
    }

    #[test]
    #[should_panic]
    fn test_project_undefined() {
        // Run test in the own environment as we do not want to affect the rest of the tests
        temp_env::with_var(CARGO_MANIFEST_DIR, None::<&str>, || {
            let _project = Project::new();
        });
    }
}
