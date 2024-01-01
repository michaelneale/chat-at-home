use std::fs::File;
use std::io::Write;
use reqwest;
use std::thread;
use std::time::Duration;
use std::process::{Command, Child};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tempfile::NamedTempFile;
use wry::Result;
use wry::WebViewBuilder;

fn main() -> Result<()> {
    let docker_compose_yaml = r#"
    version: '3.6'

    services:
      ollama:
        volumes:
          - ollama:/root/.ollama
        container_name: ollama
        pull_policy: always
        tty: true
        restart: unless-stopped
        image: ollama/ollama:latest
    
      ollama-webui:
        build:
          context: .
          args:
            OLLAMA_API_BASE_URL: '/ollama/api'
          dockerfile: Dockerfile
        image: ollama-webui:latest
        container_name: ollama-webui
        volumes:
          - ollama-webui:/app/backend/data
        depends_on:
          - ollama
        ports:
          - 3000:8080
        environment:
          - "OLLAMA_API_BASE_URL=http://ollama:11434/api"
        extra_hosts:
          - host.docker.internal:host-gateway
        restart: unless-stopped
    
    volumes:
      ollama: {}
      ollama-webui: {}
"#;

    // Create and write to a temporary Docker Compose file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(temp_file, "{}", docker_compose_yaml).expect("Failed to write to temporary file");
    let temp_file_path = temp_file.path().to_str().unwrap().to_string();

    // Start Docker Compose
    let mut docker_compose_process = Command::new("docker-compose")
        .args(&["-f", &temp_file_path, "up", "-d"])
        .spawn()
        .expect("Failed to start docker-compose");

    // Polling to check if the server is up
    let client = reqwest::blocking::Client::new();
    let mut retries = 0;
    let max_retries = 10;
    let retry_interval = Duration::from_secs(3);
    while retries < max_retries {
        if let Ok(response) = client.get("http://localhost:3000").send() {
            if response.status().is_success() {
                break;
            }
        }
        retries += 1;
        thread::sleep(retry_interval);
    }
    if retries >= max_retries {
        panic!("Failed to connect to the server");
    }        

    // Setup Tauri application
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let builder = setup_webview_builder(&window);
    let _webview = builder.with_url("http://localhost:3000")?.build()?;

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            // Stop Docker Compose when the window is closed
            Command::new("docker-compose")
                .args(&["-f", &temp_file_path, "down"])
                .status()
                .expect("Failed to stop docker-compose");

            // Clean up the Docker Compose process
            let _ = docker_compose_process.kill();
            let _ = docker_compose_process.wait();

            *control_flow = ControlFlow::Exit;
        }
    });
}

#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
))]
fn setup_webview_builder(window: &tao::window::Window) -> WebViewBuilder {
    WebViewBuilder::new(window)
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
fn setup_webview_builder(window: &tao::window::Window) -> WebViewBuilder {
    use tao::platform::unix::WindowExtUnix;
    use wry::WebViewBuilderExtUnix;
    let vbox = window.default_vbox().unwrap();
    WebViewBuilder::new_gtk(vbox)
}
