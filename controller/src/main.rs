use anyhow::anyhow;
use anyhow::{Context, Result};
use config::Config;
use crossbeam_channel::{select, unbounded};
use notify::{Event, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::{fs::read_to_string, path::Path};

pub mod config;
pub mod ipc;
pub mod modules;

const CONFIG_FILE: &str = "./config.yaml";

fn read_config(path: &str) -> Result<config::Config> {
    match read_to_string(path) {
        Ok(config) => Ok(serde_yaml::from_str::<Config>(&config)?),
        Err(_) => {
            fs::write(&path, serde_yaml::to_string(&Config::default())?)?;
            read_config(path)
        }
    }
}

// read_and_watch_config reads the config file and watches it for changes.
// If the config file changes, it will be read again and the new config will be sent over the channel.
// If there is an error, the error will be sent over the other channel.
// It is important to hold onto the INotifyWatcher so that it doesn't get dropped and die.
fn read_and_watch_config(
    path: &'static str,
) -> Result<(
    INotifyWatcher,
    crossbeam_channel::Receiver<Config>,
    crossbeam_channel::Receiver<anyhow::Error>,
)> {
    let (s, r) = unbounded();
    let (error_s, error_r) = unbounded::<anyhow::Error>();
    s.send(read_config(path)?)?;
    let mut watcher = RecommendedWatcher::new(move |res: notify::Result<Event>| match res {
        Ok(event) => {
            if !event.kind.is_modify() {
                return;
            }
            if let Ok(new_config) = read_config(path) {
                if let Err(e) = s.send(new_config) {
                    error_s.send(e.into()).unwrap();
                }
            }
        }
        Err(err) => {
            error_s
                .send(anyhow!(err).context("failed to get fs event"))
                .unwrap();
        }
    })?;
    watcher.watch(Path::new(path), RecursiveMode::NonRecursive)?;
    Ok((watcher, r, error_r))
}

fn main() -> Result<()> {
    let (_watcher, cfg_chan, err_chan) = read_and_watch_config(CONFIG_FILE)
        .with_context(|| format!("Failed to read config file {}", CONFIG_FILE))?;

    let mut modules: Vec<Box<dyn modules::Module>> = vec![];

    loop {
        select! {
          default() => {
            if let Err(err) = modules.iter().map(|m| m.tick().with_context(|| format!("error ticking {:?}", m.name()))).collect::<Result<()>>() {
                println!("{:?}", err);
            }
          },
          recv(cfg_chan) -> received => {
            let config = received?;
            let new_modules: Result<Vec<Box<dyn modules::Module>>> = config.modules.iter().map(|m| match m {
              config::Module::Camera { path } => modules::camera::Camera::new(path.to_path_buf()).map(|m| Box::new(m) as Box<dyn modules::Module>),
            }).collect();
            modules = new_modules?;
          },
          recv(err_chan) -> err => {
            println!("{:?}", err);
          },
        }
    }
}
