fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/greeter/v1/say_hello.proto")?;
    Ok(())
}