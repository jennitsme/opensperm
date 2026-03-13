#[cfg(target_os = "linux")]
pub fn apply_seccomp() -> Result<(), String> {
    // Placeholder for seccomp profile enforcement.
    // In a real build, load a seccomp-bpf profile or use libseccomp.
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn apply_seccomp() -> Result<(), String> { Ok(()) }
