use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use raylib::{
    ffi::Rectangle, prelude::RaylibDrawHandle, rgui::RaylibDrawGui, rstr, RaylibHandle,
    RaylibThread,
};

const AI_MODEL: &str = "llama3:latest";
const MAP_GENERATOR_BUTTON_X: f32 = 50.0;
const MAP_GENERATOR_BUTTON_Y: f32 = 50.0;
const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 30.0;

pub fn dev_pannel_scene(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) {
    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
    if d.gui_button(
        Rectangle {
            x: MAP_GENERATOR_BUTTON_X,
            y: MAP_GENERATOR_BUTTON_Y,
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
        },
        Some(rstr!("Generate Map")),
    ) {
        generate_map();
    }
}

fn generate_map() {
    println!("GENERATING MAP...");
    let ai_client = AiClient {
        ollama: Ollama::default(),
        model: AI_MODEL.to_string(),
    };
    println!("{}", ai_client.generate_map().unwrap());
}

struct AiClient {
    ollama: Ollama,
    model: String,
}

impl AiClient {
    fn generate_map(&self) -> Result<String, String>{
        let prompt = "Why is the sky blue ?".to_string();
        let res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { self.ollama.generate(GenerationRequest::new(self.model.clone(), prompt)).await });
        match res {
            Ok(res) => Ok(res.response),
            Err(error) => Err(format!("Error calling ai agent while generating map : {}", error)),
        }
    }
}
