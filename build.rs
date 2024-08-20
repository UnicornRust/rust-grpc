use std:: { error::Error, env, path::PathBuf };

fn main() -> Result<(), Box<dyn Error>> {
    // 获取环境变量, 输出对应的 proto 编译后的信息
    // OUT_DIR    -- the folder in which all output and intermediate artifacts
    // should be placed, This folder is inside the build directory for the package
    // being built, and it is unique for package in question
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("calculator_descriptor.bin"))
        .compile(&["proto/calculator.proto"], &["proto"])?;
    tonic_build::compile_protos("proto/calculator.proto")?;
    Ok(())
}
