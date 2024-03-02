use ollama_rs::{
    generation::completion::{request::GenerationRequest,GenerationContext},
    Ollama
};
use std::{
    io::Write,
    path::Path,
    fs,
    env
};
use bincode;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let model = &args[1];
    let mut prompt = &args[2];
    let mut reply = "";
    if args.len() == 4 {
        reply = &args[2];
        prompt = &args[3];
    }
    
    let mut request = GenerationRequest::new(model.to_string(), prompt.to_string());
    
    let valid_reply = "-r";
    if reply == valid_reply && Path::new("/tmp/hey-context").exists() {
        let data = fs::read("/tmp/hey-context").expect("Unable to read file");
        let decoded: Option<GenerationContext> = bincode::deserialize(&data[..]).unwrap();
        if let Some(context) = decoded {
            request = request.context(context);
        }
    }
    
    let ollama = Ollama::default();
    let res = ollama.generate(request).await;

    if let Ok(res) = res {
        println!("{}", res.response);
        if let Some(final_data) = res.final_data {
            let new_context = Some(final_data.context);
            let encoded: Vec<u8> = bincode::serialize(&new_context).unwrap();
            let mut f = fs::File::create("/tmp/hey-context").expect("Unable to create file");
            f.write_all(&encoded).expect("Unable to write data");
        }
    }
}