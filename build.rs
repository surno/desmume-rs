use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn env(name: &str) -> Option<String> {
    let prefix = env::var("TARGET").unwrap().to_uppercase().replace("-", "_");
    let prefixed = format!("{}_{}", prefix, name);
    println!("cargo:rerun-if-env-changed={}", prefixed);

    if let Ok(var) = env::var(&prefixed) {
        return Some(var);
    }

    println!("cargo:rerun-if-env-changed={}", name);
    env::var(name).ok()
}

fn main() {
    let target = env::var("TARGET").unwrap();

    if cfg!(feature = "desmume-system") || env("DESMUME_SYSTEM").is_some() {
        return;
    }

    let src = env::current_dir().unwrap();
    let build_dir = TempDir::new().unwrap();
    let build_dir = build_dir.path();

    // Copy sources over
    let mut cmd = Command::new("cp");
    cmd.current_dir(&build_dir)
        .arg("-a")
        .arg(&src.join("desmume/desmume/src"))
        .arg(&build_dir);
    run(&mut cmd, "cp");

    if target.contains("windows") {
        // MSVC-based Windows build
        let mut cmd = Command::new("MSBuild.exe");
        let config = if env("FASTBUILD").is_some() {
            "Release Fastbuild"
        } else {
            "Release"
        };
        let (arch_dirname, arch_targetname) = if cfg!(target_pointer_width = "64") {
            ("x64", "x64")
        } else {
            ("x86", "Win32")
        };
        cmd
            .arg("DeSmuME_Interface.vcxproj")
            .arg(format!("/p:configuration={}", config))
            .arg(format!("/p:Platform={}", arch_targetname))
            .current_dir(&build_dir.join("src/frontend/interface/windows"));
        run(&mut cmd, "meson");

        let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let dll_path = glob::glob(&build_dir.join("src/frontend/interface/windows/__bins/*.dll").to_str().unwrap())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let mut cmd = Command::new("cp");
        cmd.current_dir(&build_dir)
            .arg(dll_path)
            .arg(&dst.join("desmume.dll"));
        let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let lib_path = glob::glob(&build_dir.join("src/frontend/interface/windows/__bins/*.lib").to_str().unwrap())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let mut cmd = Command::new("cp");
        cmd.current_dir(&build_dir)
            .arg(lib_path)
            .arg(&dst.join("desmume.lib"));
        run(&mut cmd, "cp");
        let mut cmd = Command::new("cp");
        cmd.current_dir(&build_dir)
            .arg(&build_dir.join(format!("src/frontend/interface/windows/SDL/lib/{}/SDL2.dll", arch_dirname)))
            .arg(&dst);
        run(&mut cmd, "cp");
        println!("cargo:rustc-link-search={}", dst.as_os_str().to_str().unwrap())
    } else {
        // Meson based Linux/Mac build
        let mut cmd = Command::new("meson");
        cmd
            .arg("build")
            .current_dir(&build_dir.join("src/frontend/interface"));
        run(&mut cmd, "meson");

        let mut cmd = Command::new("ninja");
        cmd
            .arg("-C")
            .arg("build")
            .current_dir(&build_dir.join("src/frontend/interface"));
        run(&mut cmd, "meson");

        if target.contains("apple-darwin") {
            let mut cmd = Command::new("mv");
            cmd.current_dir(&build_dir)
                .arg(&build_dir.join("src/frontend/interface/build/libdesmume.dylib"))
                .arg(&build_dir.join("src/frontend/interface/build/libdesmume.so"));
            run(&mut cmd, "mv");
        }

        let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let mut cmd = Command::new("cp");
        cmd.current_dir(&build_dir)
            .arg(&build_dir.join("src/frontend/interface/build/libdesmume.so"))
            .arg(&dst);
        run(&mut cmd, "cp");
    }
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!(
                "failed to execute command: {}\nis `{}` not installed?",
                e, program
            ));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!(
            "command did not execute successfully, got: {}",
            status
        ));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
