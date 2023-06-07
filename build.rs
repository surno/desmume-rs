use std::env;
use std::fs::copy;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn env(name: &str) -> Option<String> {
    let prefix = env::var("TARGET").unwrap().to_uppercase().replace('-', "_");
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
    cmd.current_dir(build_dir)
        .arg("-a")
        .arg(&src.join("desmume/desmume/src"))
        .arg(build_dir);
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
        cmd.arg("DeSmuME_Interface.vcxproj")
            .arg(format!("/p:configuration={}", config))
            .arg(format!("/p:Platform={}", arch_targetname))
            .arg("-property:name=StaticLibrary")
            .current_dir(&build_dir.join("src/frontend/interface/windows"));
        run(&mut cmd, "meson");

        let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let lib_path = glob::glob(
            build_dir
                .join("src/frontend/interface/windows/__bins/*.lib")
                .to_str()
                .unwrap(),
        )
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
        let sdl_lib_path = glob::glob(
            build_dir
                .join(format!(
                    "src/frontend/interface/windows/SDL/lib/{arch_dirname}/SDL2.lib"
                ))
                .to_str()
                .unwrap(),
        )
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
        copy(lib_path, dst.join("desmume.lib")).unwrap();
        copy(sdl_lib_path, dst.join("SDL2.lib")).unwrap();
        println!(
            "cargo:rustc-link-search={}",
            dst.as_os_str().to_str().unwrap()
        );
        println!("cargo:lib=static={}", dst.display());
    } else {
        // Meson based Linux/Mac build
        let mut cmd = Command::new("meson");
        cmd.arg("build")
            .arg("--default-library=static")
            .arg("-Dbuildtype=release")
            .current_dir(&build_dir.join("src/frontend/interface"));
        run(&mut cmd, "meson");

        let mut cmd = Command::new("ninja");
        cmd.arg("-C")
            .arg("build")
            .current_dir(&build_dir.join("src/frontend/interface"));
        run(&mut cmd, "ninja");

        let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let mut cmd = Command::new("cp");
        cmd.current_dir(build_dir)
            .arg("-r")
            .arg(&build_dir.join("src/frontend/interface/build/libdesmume.a"))
            .arg(&build_dir.join("src/frontend/interface/build/libdesmume.a.p"))
            .arg(&dst);
        run(&mut cmd, "cp");

        let cfg = pkg_config::Config::new();
        cfg.probe("glib-2.0").unwrap();
        cfg.probe("sdl2").unwrap();
        if cfg.probe("libpcap").is_err() {
            // Probing may fail under MacOS. Still try to link.
            println!("cargo:rustc-link-lib=pcap");
        }
        cfg.probe("zlib").unwrap();
        cfg.probe("soundtouch").ok();
        cfg.probe("openal").ok();
        cfg.probe("opengl").unwrap();
        cfg.probe("alsa").ok();

        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-search={}", dst.display());
        println!("cargo:lib=static={}", dst.display());
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
