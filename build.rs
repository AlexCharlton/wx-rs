use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cc;
use embed_resource;

const WX_PATH: &str = "wxWidgets-3.2.2.1";
const WX_SOURCE: &str =
    "https://github.com/wxWidgets/wxWidgets/releases/download/v3.2.2.1/wxWidgets-3.2.2.1.tar.bz2";

fn is_windows() -> bool {
    cfg!(windows)
}

fn is_msvc() -> bool {
    cfg!(target_env = "msvc")
}

fn is_osx() -> bool {
    cfg!(target_os = "macos")
}

fn main() -> Result<(), Box<dyn Error>> {
    let wx_path = download_dist()?;
    build_bridge_lib(&wx_path)?;

    if !is_windows() && !is_osx() {
        return Ok(());
    }

    let wx_libs = get_libs(&wx_path)?;

    let mut iter = wx_libs.split_whitespace();
    while let Some(flag) = iter.next() {
        if flag.starts_with("-l") {
            println!("cargo:rustc-link-lib={}", &flag[2..]);
        } else if flag.starts_with("-L") {
            println!("cargo:rustc-link-search={}", &flag[2..]);
        } else if flag == "-framework" {
            println!("cargo:rustc-link-lib=framework={}", iter.next().unwrap());
        }
    }

    if is_msvc() {
        // gunzip_libs(wx_path);
        println!("cargo:rustc-link-lib=static=wxbase32u");
        println!("cargo:rustc-link-lib=static=wxmsw32u_core");
        println!("cargo:rustc-link-lib=static=wxpng");
        println!("cargo:rustc-link-lib=static=wxjpeg");
        println!("cargo:rustc-link-lib=static=wxtiff");
        println!("cargo:rustc-link-lib=static=wxregexu");
        println!("cargo:rustc-link-lib=static=wxzlib");
        let include_path = wx_path.join("include");
        let resource_file = include_path.join("wx/msw/wx.rc");
        env::set_var("INCLUDE", include_path.to_str().unwrap());
        embed_resource::compile(resource_file);
    } else if is_windows() {
        println!("cargo:rustc-link-search=C://msys64/mingw64/lib");
        println!("cargo:rustc-link-search=C://msys64/usr/lib/w32api");
        println!("cargo:rustc-link-lib=static=wx_mswu_gl-3.2");
        println!("cargo:rustc-link-lib=static=wx_mswu_core-3.2");
        println!("cargo:rustc-link-lib=static=wx_baseu-3.2");
        println!("cargo:rustc-link-lib=static=wxpng-3.2");
        println!("cargo:rustc-link-lib=static=wxjpeg-3.2");
        println!("cargo:rustc-link-lib=static=wxtiff-3.2");
        println!("cargo:rustc-link-lib=static=wxregexu-3.2");
        embed_resource_file(&wx_path);
    }

    Ok(())
}

use bzip2::read::BzDecoder;
fn download_dist() -> Result<PathBuf, Box<dyn Error>> {
    let dist_path = Path::new("./dist/");
    let wx_path = PathBuf::from(format!("./dist/{}", WX_PATH));
    if wx_path.exists() {
        return Ok(wx_path);
    }
    println!("cargo:warning=wx-rs: Downloading source to './dist/");
    fs::create_dir_all(dist_path)?;
    let source_response = reqwest::blocking::get(WX_SOURCE)?;
    let decoder = BzDecoder::new(source_response);
    let mut archive = tar::Archive::new(decoder);
    for file in archive.entries()? {
        let mut file = file?;
        file.unpack_in(dist_path)?;
    }

    return Ok(wx_path);
}

fn build_bridge_lib(wx_path: &Path) -> Result<(), Box<dyn Error>> {
    if !is_windows() && !is_osx() {
        println!("cargo:warning=wx-rs: Platform unsupported. Building a stub library.");
        println!("cargo:rerun-if-changed=cpp_src/wxstub.cpp");
        cc::Build::new()
            .cpp(true)
            .file("cpp_src/wxstub.cpp")
            .compile("libwxbridge.a");
        return Ok(());
    }
    println!("cargo:rerun-if-changed=cpp_src/wxbridge.cpp");

    //panic!("{}", wx_path.to_str().unwrap());
    if is_windows() {
        build_msvc(wx_path)?;
    } else {
        panic!("Not supported")
    };

    let wx_flags = if is_msvc() {
        format!(
            "-I{}/lib/vc_x64_lib/mswu -I{}/include -D_WIN64 -D_FILE_OFFSET_BITS=64 -D__WXMSW__ -D_UNICODE -DNDEBUG -DNOPCH  /GR /EHsc",
            wx_path.display(),
            wx_path.display(),
        )
    } else if is_windows() {
        format!("-I{}/msw64-release-build/lib/wx/include/msw-unicode-static-3.1 -I{}/include -D_FILE_OFFSET_BITS=64 -D__WXMSW__", wx_path.display(), wx_path.display())
    } else if is_osx() {
        String::from_utf8(
            Command::new(format!("{}/osx-release-build/wx-config", wx_path.display()))
                .arg("--cxxflags")
                .output()?
                .stdout,
        )?
    } else {
        panic!("Unsupported platform")
    };

    env::set_var("CXXFLAGS", wx_flags);

    cc::Build::new()
        .cpp(true)
        .file("cpp_src/wxbridge.cpp")
        .shared_flag(false)
        .compile("libwxbridge.a");

    Ok(())
}

fn build_msvc(wx_path: &Path) -> Result<(), Box<dyn Error>> {
    if wx_path
        .join("lib")
        .join("vc_x64_lib")
        .join("wxmsw32u_core.lib")
        .exists()
    {
        println!("cargo:warning=wx-rs: Already built wxWidgets. Skipping a re-build.");
        return Ok(());
    }
    println!("cargo:warning=wx-rs: Building wxWidgets. This can take a few minutes.");
    let status = Command::new("nmake")
        .current_dir(wx_path.join("build").join("msw"))
        .args([
            "-f",
            "makefile.vc",
            "-a",
            "BUILD=release",
            "TARGET_CPU=X64",
            "USE_STC=0",
            "USE_OPENGL=0",
            "USE_HTML=0",
            "USE_AUI=0",
            "USE_MEDIA=0",
            "USE_PROPGRID=0",
            "USE_QA=0",
            "USE_RIBBON=0",
            "USE_WEBVIEW=0",
            "USE_XRC=0",
        ])
        .output()?;
    let stdout = std::str::from_utf8(&status.stdout)?.to_string();
    let stderr = std::str::from_utf8(&status.stderr)?.to_string();
    println!("Building: {}", stdout);
    assert!(status.status.success(), "{}", stderr);

    Ok(())
}

fn get_libs(wx_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    if is_msvc() {
        msvc_libs(wx_path)
    } else if is_windows() {
        windows_libs(wx_path)
    } else if is_osx() {
        osx_libs(wx_path)
    } else {
        panic!("Unsupported platform")
    }
}

fn msvc_libs(wx_path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(format!(
        "-L{} -lrpcrt4 -loleaut32 -lole32 -luuid -lwinspool -lwinmm -lshell32 -lcomctl32 -lcomdlg32 -ladvapi32 -lwsock32 -lgdi32 -loleacc -lversion -luxtheme -lshlwapi -luser32",
        &wx_path.join("lib").join("vc_x64_lib").canonicalize()?.to_str().unwrap()[4..],
    ))
}

fn windows_libs(wx_path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(format!("-L{}/msw64-release-build/lib -lopengl32 -lwxtiff-3.1 -lwxjpeg-3.1 -lwxpng-3.1 -lwxregexu-3.1 -lwxscintilla-3.1 -lrpcrt4 -loleaut32 -lole32 -luuid -lwinspool -lwinmm -lshell32 -lcomctl32 -lcomdlg32 -ladvapi32 -lwsock32 -lgdi32 -lexpat -lz  -loleacc -lversion -luxtheme -lshlwapi -luser32", &wx_path.canonicalize()?.to_str().unwrap()[4..]))
}

fn osx_libs(wx_path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(String::from_utf8(
        Command::new(format!("{}/osx-release-build/wx-config", wx_path.display()))
            .arg("--libs")
            .arg("core,base,gl")
            .output()?
            .stdout,
    )?)
}

fn embed_resource_file(wx_path: &PathBuf) {
    /*
    This was cribbed from https://github.com/nabijaczleweli/rust-embed-resource
    but extended to support the `--include-dir` argument to windres
    */
    let include_path = wx_path.join("include");
    let resource_file = include_path.join("wx/msw/wx.rc");

    let prefix = &resource_file
        .file_stem()
        .expect("resource_file has no stem")
        .to_str()
        .expect("resource_file's stem not UTF-8");
    let out_dir = env::var("OUT_DIR").expect("No OUT_DIR env var");

    let resource = resource_file.to_str().unwrap();
    let out_file = format!("{}/lib{}.a", out_dir, prefix);

    // https://sourceware.org/binutils/docs/binutils/windres.html
    match Command::new("windres")
        .args(&[
            "--include-dir",
            include_path.to_str().unwrap(),
            "--input",
            resource,
            "--output-format=coff",
            "--output",
            &out_file,
        ])
        .status()
    {
        Ok(stat) if stat.success() => {}
        Ok(stat) => panic!(
            "windres failed to compile \"{}\" into \"{}\" with {}",
            resource, out_file, stat
        ),
        Err(e) => panic!(
            "Couldn't execute windres to compile \"{}\" into \"{}\": {}",
            resource, out_file, e
        ),
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib={}", prefix);
}
