use std::env;
use std::error::Error;
use std::path::Path;
use std::process::Command;

use cc;
use embed_resource;

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
    if !is_windows() && !is_osx() {
        println!("Platform unsupported. Building a stub library.");
        println!("cargo:rerun-if-changed=cpp_src/wxstub.cpp");
        cc::Build::new()
            .cpp(true)
            .file("cpp_src/wxstub.cpp")
            .compile("libwxbridge.a");
        return Ok(());
    }
    println!("cargo:rerun-if-changed=cpp_src/wxbridge.cpp");

    let wx_path = Path::new("./dist/wxWidgets/");
    //panic!("{}", wx_path.to_str().unwrap());

    let wx_flags = if is_msvc() {
        format!(
            "-I{}lib/vc_x64_lib/mswu -I{}include -D_WIN64 -D_FILE_OFFSET_BITS=64 -D__WXMSW__ -D_UNICODE -DNDEBUG -DNOPCH  /GR /EHsc",
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

    let wx_libs = if is_msvc() {
        format!(
            "-L{}/lib/vc_x64_lib -lrpcrt4 -loleaut32 -lole32 -luuid -lwinspool -lwinmm -lshell32 -lcomctl32 -lcomdlg32 -ladvapi32 -lwsock32 -lgdi32 -loleacc -lversion -luxtheme -lshlwapi -luser32",
            &wx_path.canonicalize()?.to_str().unwrap()[4..]
        )
    } else if is_windows() {
        format!("-L{}/msw64-release-build/lib -lopengl32 -lwxtiff-3.1 -lwxjpeg-3.1 -lwxpng-3.1 -lwxregexu-3.1 -lwxscintilla-3.1 -lrpcrt4 -loleaut32 -lole32 -luuid -lwinspool -lwinmm -lshell32 -lcomctl32 -lcomdlg32 -ladvapi32 -lwsock32 -lgdi32 -lexpat -lz  -loleacc -lversion -luxtheme -lshlwapi -luser32", &wx_path.canonicalize()?.to_str().unwrap()[4..])
    } else if is_osx() {
        String::from_utf8(
            Command::new(format!("{}/osx-release-build/wx-config", wx_path.display()))
                .arg("--libs")
                .arg("core,base,gl")
                .output()?
                .stdout,
        )?
    } else {
        panic!("Unsupported platform")
    };

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
        gunzip_libs(wx_path);
        println!("cargo:rustc-link-lib=static=wxbase31u");
        println!("cargo:rustc-link-lib=static=wxmsw31u_core");
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
        println!("cargo:rustc-link-lib=static=wx_mswu_gl-3.1");
        println!("cargo:rustc-link-lib=static=wx_mswu_core-3.1");
        println!("cargo:rustc-link-lib=static=wx_baseu-3.1");
        println!("cargo:rustc-link-lib=static=wxpng-3.1");
        println!("cargo:rustc-link-lib=static=wxjpeg-3.1");
        println!("cargo:rustc-link-lib=static=wxtiff-3.1");
        println!("cargo:rustc-link-lib=static=wxregexu-3.1");
        embed_resource_file(&wx_path);
    }

    Ok(())
}

use flate2::read::GzDecoder;
use glob::glob;
use std::fs::File;
use std::io;
fn gunzip_libs(wx_path: &Path) {
    let path = wx_path.join("lib/vc_x64_lib");
    for entry in glob(&format!("{}/*.gz", path.display())).unwrap() {
        if let Ok(file_path) = entry {
            if path.join(file_path.file_stem().unwrap()).exists() {
                continue;
            }
            let gz_file = File::open(&file_path).unwrap();
            let mut gz = GzDecoder::new(&gz_file);
            let mut out = File::create(path.join(file_path.file_stem().unwrap())).unwrap();
            io::copy(&mut gz, &mut out).unwrap();
        }
    }
}

fn embed_resource_file(wx_path: &Path) {
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
