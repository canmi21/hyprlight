use anyhow::{anyhow, Context, Result};
use clap::{ArgAction, Parser};
use notify_rust::Notification;
use std::{
    env,
    process::{Command, Stdio},
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

#[derive(Parser, Debug)]
#[command(
    name = "hyprlight",
    version,
    about = "Brightness control utility for Hyprland",
    after_help = "Example:
    hyprlight i 10    # Increase brightness by 10%
    hyprlight d       # Decrease brightness by default step (5%)
    hyprlight i 5 -q  # Increase brightness by 5% quietly"
)]
struct Args {
    #[command(subcommand)]
    action: Action,

    #[arg(default_value_t = 5)]
    step: u32,

    #[arg(short, long, action = ArgAction::SetFalse)]
    notify: bool,
}

#[derive(clap::Subcommand, Debug, Clone, Copy)]
enum Action {
    I,
    D,
}

fn main() -> Result<()> {
    let args = Args::parse();

    check_existing_instances()?;

    let use_swayosd = check_swayosd()?;

    let (device_name, current_brightness) = get_current_brightness()?;

    let adjusted_step = adjust_step(current_brightness, args.step, args.action);

    if use_swayosd {
        adjust_with_swayosd(adjusted_step, args.action)?;
    } else {
        adjust_with_brightnessctl(adjusted_step, args.action, current_brightness)?;
    }

    if args.notify {
        send_notification(device_name, current_brightness)?;
    }

    Ok(())
}

fn check_existing_instances() -> Result<()> {
    let system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new()),
    );
    let current_pid = sysinfo::get_current_pid().unwrap();
    let process_name = env::args().next().unwrap_or_default();

    let count = system.processes().values().filter(|p| {
        p.exe().and_then(|exe| exe.to_str()).unwrap_or_default().ends_with(&process_name) && p.pid() != current_pid
    }).count();

    if count > 0 {
        return Err(anyhow!("An instance of the script is already running..."));
    }
    Ok(())
}

fn check_swayosd() -> Result<bool> {
    let swayosd_client_exists = Command::new("which")
        .arg("swayosd-client")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())?;

    if !swayosd_client_exists {
        return Ok(false);
    }

    let mut system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new()),
    );
    system.refresh_processes();

    let server_exists = system
        .processes_by_exact_name("swayosd-server")
        .next()
        .is_some();
    
    Ok(server_exists)
}

fn get_current_brightness() -> Result<(String, u32)> {
    let output = Command::new("brightnessctl")
        .args(["info", "-m"])
        .output()
        .context("Failed to execute brightnessctl")?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = output_str.split(',').collect();

    if parts.len() < 4 {
        return Err(anyhow!("Failed to parse brightnessctl output"));
    }

    let device_name = parts[0].to_string();
    let brightness_str = parts[3].trim_end_matches('%');
    let brightness: u32 = brightness_str.parse().context("Failed to parse brightness value")?;

    Ok((device_name, brightness))
}

fn adjust_step(current: u32, step: u32, action: Action) -> u32 {
    match action {
        Action::I if current < 10 => 1,
        Action::D if current <= 10 => 1,
        _ => step,
    }
}

fn adjust_with_swayosd(step: u32, action: Action) -> Result<()> {
    let cmd = match action {
        Action::I => "raise",
        Action::D => "lower",
    };

    Command::new("swayosd-client")
        .args(["--brightness", cmd, &step.to_string()])
        .status()
        .context("Failed to execute swayosd-client")?;

    Ok(())
}

fn adjust_with_brightnessctl(step: u32, action: Action, current: u32) -> Result<()> {
    let arg = match action {
        Action::I => format!("+{step}%"),
        Action::D if current <= 1 => format!("{step}%"),
        Action::D => format!("{step}%-"),
    };

    Command::new("brightnessctl")
        .arg("set")
        .arg(&arg)
        .status()
        .context("Failed to execute brightnessctl")?;

    Ok(())
}

fn send_notification(device_name: String, current: u32) -> Result<()> {
    let home = env::var("HOME").context("Failed to get HOME directory")?;
    let angle = ((current + 2) / 5) * 5;
    let icon = format!("{}/.config/dunst/icons/vol/vol-{}.svg", home, angle);

    let bar = "•".repeat((current as f32 / 15.0).ceil() as usize);
    let summary = format!("{}% {}", current, bar);

    Notification::new()
        .id(987654321)
        .summary(&summary)
        .body(&device_name)
        .icon(&icon)
        .timeout(800)
        .show()
        .context("Failed to send notification")?;

    Ok(())
}
