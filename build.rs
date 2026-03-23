fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/greeter/v1/say_hello.proto")?;
    tonic_prost_build::compile_protos("proto/fact/v1/chuck_norris.proto")?;
    Ok(())
}