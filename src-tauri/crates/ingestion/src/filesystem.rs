use promoforge_core::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const IGNORED_DIRS: &[&str] = &[
    "node_modules", ".git", "__pycache__", ".venv", "venv", "env",
    "build", "dist", "target", ".next", ".nuxt", "build", "out",
    ".cache", ".parcel-cache", "coverage", ".nyc_output", ".sass-cache",
    "bower_components", "vendor", ".terraform", ".vagrant",
];

const IGNORED_FILES: &[&str] = &[
    ".env", ".env.local", ".env.production",
    "package-lock.json", "yarn.lock", "pnpm-lock.yaml",
    "Cargo.lock", "poetry.lock", "Pipfile.lock",
    ".DS_Store", "Thumbs.db", "desktop.ini",
];

const IGNORED_EXTENSIONS: &[&str] = &[
    "exe", "dll", "so", "dylib", "o", "obj",
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp",
    "mp3", "mp4", "wav", "ogg", "flac", "avi", "mov", "mkv",
    "zip", "tar", "gz", "rar", "7z",
    "ttf", "otf", "woff", "woff2",
    "pdf", "doc", "docx", "xls", "xlsx",
    "woff", "woff2", "eot",
];

const EXTENSION_LANG_MAP: &[(&str, &str)] = &[
    ("ts", "TypeScript"), ("tsx", "TypeScript"),
    ("js", "JavaScript"), ("jsx", "JavaScript"), ("mjs", "JavaScript"), ("cjs", "JavaScript"),
    ("py", "Python"), ("pyi", "Python"),
    ("rs", "Rust"),
    ("go", "Go"),
    ("java", "Java"),
    ("kt", "Kotlin"), ("kts", "Kotlin"),
    ("cs", "C#"),
    ("swift", "Swift"),
    ("rb", "Ruby"),
    ("php", "PHP"),
    ("c", "C"), ("h", "C"),
    ("cpp", "C++"), ("cc", "C++"), ("cxx", "C++"), ("hpp", "C++"),
    ("zig", "Zig"),
    ("lua", "Lua"),
    ("dart", "Dart"),
    ("scala", "Scala"), ("sc", "Scala"),
    ("ex", "Elixir"), ("exs", "Elixir"),
    ("hs", "Haskell"),
    ("r", "R"),
    ("jl", "Julia"),
    ("toml", "TOML"), ("yaml", "YAML"), ("yml", "YAML"),
    ("json", "JSON"), ("jsonc", "JSON"),
    ("html", "HTML"), ("htm", "HTML"),
    ("css", "CSS"), ("scss", "SCSS"), ("less", "LESS"),
    ("sql", "SQL"),
    ("graphql", "GraphQL"), ("gql", "GraphQL"),
    ("proto", "Protocol Buffers"),
    ("tf", "Terraform"), ("hcl", "Terraform"),
    ("dockerfile", "Dockerfile"),
    ("sh", "Shell"), ("bash", "Shell"), ("zsh", "Shell"),
    ("ps1", "PowerShell"),
    ("md", "Markdown"), ("mdx", "MDX"),
];

pub struct FilesystemWalker;

impl FilesystemWalker {
    pub fn scan(root: &str) -> Result<FileManifest> {
        let root_path = Path::new(root);
        if !root_path.exists() {
            return Err(Error::InvalidInput(format!(
                "Path does not exist: {}",
                root
            )));
        }

        let mut files = Vec::new();
        let mut languages_detected: HashMap<String, usize> = HashMap::new();
        let mut total_size: u64 = 0;

        Self::walk_recursive(root_path, root_path, &mut files, &mut languages_detected, &mut total_size)?;

        let mut lang_vec: Vec<(String, usize)> = languages_detected
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();
        lang_vec.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(FileManifest {
            root: root.to_string(),
            total_files: files.len(),
            total_size,
            languages_detected: lang_vec.into_iter().map(|(k, _)| k).collect(),
            files,
        })
    }

    fn walk_recursive(
        dir: &Path,
        root: &Path,
        files: &mut Vec<FileEntry>,
        languages: &mut HashMap<String, usize>,
        total_size: &mut u64,
    ) -> Result<()> {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if path.is_dir() {
                if IGNORED_DIRS.contains(&name.as_str()) || name.starts_with('.') {
                    continue;
                }
                Self::walk_recursive(&path, root, files, languages, total_size)?;
                continue;
            }

            if IGNORED_FILES.contains(&name.as_str()) {
                continue;
            }

            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if IGNORED_EXTENSIONS.contains(&ext) {
                    continue;
                }
            }

            let metadata = match std::fs::metadata(&path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let size = metadata.len();
            *total_size += size;

            let relative_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            let language = EXTENSION_LANG_MAP
                .iter()
                .find(|(ext, _)| extension.eq_ignore_ascii_case(ext))
                .map(|(_, lang)| lang.to_string());

            if let Some(ref lang) = language {
                *languages.entry(lang.clone()).or_insert(0) += 1;
            }

            let category = Self::categorize_file(&path, &name, extension);
            let line_count = if category == FileCategory::SourceCode || category == FileCategory::Configuration {
                Self::count_lines(&path)
            } else {
                None
            };

            files.push(FileEntry {
                path: path.to_string_lossy().to_string(),
                relative_path,
                category,
                language,
                size,
                line_count,
            });
        }

        Ok(())
    }

    fn categorize_file(path: &Path, name: &str, ext: &str) -> FileCategory {
        let name_lower = name.to_lowercase();
        let path_str = path.to_string_lossy().to_lowercase();

        if name_lower == "license" || name_lower == "license.md" || name_lower == "license.txt" || ext == "license" {
            return FileCategory::License;
        }

        if name_lower == "readme.md" || name_lower == "readme.txt"
            || name_lower.starts_with("readme") || path_str.contains("/docs/") || path_str.contains("/doc/")
        {
            return FileCategory::Documentation;
        }

        if name_lower == "test" || path_str.contains("/test/") || path_str.contains("/tests/")
            || path_str.contains("/__tests__/") || name_lower.ends_with(".test.ts")
            || name_lower.ends_with(".test.js") || name_lower.ends_with("_test.rs")
            || name_lower.ends_with("_test.go") || name_lower.ends_with("test_*.py")
        {
            return FileCategory::Test;
        }

        if name_lower == "package.json" || name_lower == "cargo.toml" || name_lower == "go.mod"
            || name_lower == "requirements.txt" || name_lower == "pyproject.toml"
            || name_lower == "pom.xml" || name_lower == "build.gradle"
            || name_lower == "pubspec.yaml" || name_lower == "gemfile"
            || name_lower == "composer.json" || name_lower == "setup.py"
            || name_lower == "setup.cfg" || name_lower == "poetry.lock"
            || name_lower == "yarn.lock" || name_lower == "package-lock.json"
        {
            return FileCategory::Dependency;
        }

        if name_lower == "dockerfile" || name_lower == "docker-compose.yml"
            || name_lower == ".github" || path_str.contains("/.github/")
            || name_lower == "makefile" || name_lower == "justfile"
            || name_lower == ".gitlab-ci.yml" || name_lower == "Jenkinsfile"
            || name_lower == "cloudbuild.yaml" || name_lower == "bitbucket-pipelines.yml"
        {
            return FileCategory::CiCd;
        }

        if name_lower == "tsconfig.json" || name_lower == "vite.config.ts"
            || name_lower == "webpack.config.js" || name_lower == "next.config.js"
            || name_lower == "tailwind.config.js" || name_lower == ".eslintrc"
            || name_lower == ".prettierrc" || name_lower == "jest.config.js"
            || name_lower == "vitest.config.ts" || name_lower == ".cargo"
            || ext == "toml" || ext == "yaml" || ext == "yml"
            || name_lower == "settings.json" || name_lower == ".editorconfig"
        {
            return FileCategory::Configuration;
        }

        if path_str.contains("/build/") || path_str.contains("/dist/")
            || path_str.contains("/target/") || path_str.contains("/out/")
        {
            return FileCategory::Build;
        }

        let source_extensions = [
            "ts", "tsx", "js", "jsx", "py", "rs", "go", "java", "kt",
            "cs", "swift", "rb", "php", "c", "cpp", "h", "hpp", "zig",
            "lua", "dart", "scala", "ex", "hs", "r", "jl", "sql", "graphql",
        ];
        if source_extensions.contains(&ext) {
            return FileCategory::SourceCode;
        }

        FileCategory::Other
    }

    fn count_lines(path: &Path) -> Option<usize> {
        let content = std::fs::read_to_string(path).ok()?;
        Some(content.lines().count())
    }
}
