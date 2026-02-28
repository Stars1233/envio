#[cfg(feature = "application")]
mod application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "application")]
    {
        application::export_completion_paths()?;
        application::export_build_env_vars();
    }

    Ok(())
}
