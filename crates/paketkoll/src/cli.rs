use clap::Parser;
use clap::Subcommand;
use compact_str::CompactString;
use std::fmt::Display;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[clap(disable_help_subcommand = true)]
pub struct Cli {
    /// Trust mtime (don't check checksum if it matches)
    #[arg(long)]
    pub trust_mtime: bool,
    /// Include config files in the check
    #[arg(long, default_value_t = ConfigFiles::Exclude)]
    pub config_files: ConfigFiles,
    /// Which package manager backend to use
    #[arg(short, long, default_value_t = Backend::Auto)]
    pub backend: Backend,
    /// Output format to use
    #[arg(short, long, default_value_t = Format::Human)]
    pub format: Format,
    /// Paths to ignore (apart from built in ones). Basic globs are
    /// supported. Use ** to match any number of path components.
    #[arg(long)]
    pub ignore: Vec<CompactString>,
    /// Operation to perform
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check package files
    Check {
        /// Packages to check (default: all of them)
        packages: Vec<String>,
    },
    /// Check package files and search for unexpected files
    CheckUnexpected {
        /// Should paths be canonicalized before checking? If you get many false
        /// positives, try this. Required on Debian due to lack of /usr
        /// merge.
        #[arg(long)]
        canonicalize: bool,
    },
    /// Get a list of installed packages
    InstalledPackages,
    /// Find package that owns a given file.
    Owns {
        /// Path to query
        paths: Vec<String>,
    },
    /// Get the original content of a file
    #[clap(hide = true)]
    OriginalFile {
        /// Package to query
        #[arg(long)]
        package: Option<String>,
        /// Path to query
        path: String,
    },
    #[clap(hide = true)]
    DebugPackageFileData {
        /// Package to query
        package: String,
    },
}

/// Output format to use
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, clap::ValueEnum)]
pub enum Format {
    /// Human-readable output
    Human,
    /// JSON formatted output
    #[cfg(feature = "json")]
    Json,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Human => write!(f, "human"),
            #[cfg(feature = "json")]
            Self::Json => write!(f, "json"),
        }
    }
}

/// Determine which package manager backend to use
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, clap::ValueEnum)]
pub enum Backend {
    /// Select based on current distro
    Auto,
    /// Backend for Arch Linux and derived distros (pacman)
    #[cfg(feature = "arch_linux")]
    ArchLinux,
    /// Backend for Debian and derived distros (dpkg/apt)
    #[cfg(feature = "debian")]
    Debian,
    /// Backend for Flatpak (EXPERIMENTAL)
    Flatpak,
    /// Backend for systemd-tmpfiles (EXPERIMENTAL)
    #[cfg(feature = "systemd_tmpfiles")]
    SystemdTmpfiles,
}

impl Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            #[cfg(feature = "arch_linux")]
            Self::ArchLinux => write!(f, "arch-linux"),
            #[cfg(feature = "debian")]
            Self::Debian => write!(f, "debian"),
            Self::Flatpak => write!(f, "flatpak"),
            #[cfg(feature = "systemd_tmpfiles")]
            Self::SystemdTmpfiles => write!(f, "systemd-tmpfiles"),
        }
    }
}

/// Describe how to check config files
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, clap::ValueEnum)]
pub enum ConfigFiles {
    /// Include config files in check
    Include,
    /// Exclude config files in check
    Exclude,
    /// Only check config files
    Only,
}

impl Display for ConfigFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Include => write!(f, "include"),
            Self::Exclude => write!(f, "exclude"),
            Self::Only => write!(f, "only"),
        }
    }
}
