//! SigmaOS-omarchy-update — Omarchy updater CLI.
//!
//! Usage:
//!   omarchy-update status
//!   omarchy-update channel [stable|beta|edge]
//!   omarchy-update check
//!   omarchy-update list
//!   omarchy-update apply <id>
//!   omarchy-update apply-all
//!   omarchy-update rollback <id>
//!   omarchy-update reboot-needed

use omarchy_update::{AvailableUpdate, Channel, OmarchyUpdate, UpdateKind};
use std::process::exit;

fn seed() -> OmarchyUpdate {
    let mut m = OmarchyUpdate::new();
    let ups = [
        ("linux", "linux", UpdateKind::SystemPackage, Channel::Stable, false, None),
        ("mesa", "mesa", UpdateKind::SystemPackage, Channel::Stable, false, None),
        ("omarchy-core", "omarchy-core", UpdateKind::OmarchyComponent, Channel::Stable, false, Some("mig-omarchy-1.2")),
        ("hyprland-git", "hyprland-git", UpdateKind::SystemPackage, Channel::Beta, false, None),
        ("fw-nvme", "linux-firmware-nvme", UpdateKind::Firmware, Channel::Edge, true, None),
        ("waybar-theme", "omarchy-waybar-theme", UpdateKind::Dotfiles, Channel::Stable, false, None),
    ];
    for (id, name, kind, ch, reboot, mig) in ups {
        m.offer(AvailableUpdate {
            id: id.into(), name: name.into(), kind,
            from_version: "1.0".into(), to_version: "1.1".into(),
            channel: ch, requires_reboot: reboot, migration: mig.map(|s| s.to_string()),
        });
    }
    m
}

fn usage() -> ! {
    eprintln!("usage: omarchy-update <status|channel|check|list|apply|apply-all|rollback|reboot-needed> [args]");
    exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() { usage(); }
    let mut m = seed();
    let cmd = args[0].as_str();
    let rest = &args[1..];

    let result: Result<(), String> = (|| -> Result<(), String> { match cmd {
        "status" => { println!("{}", m.status_line()); Ok(()) }
        "channel" => {
            let c = rest.first().map(|s| s.as_str()).unwrap_or_else(|| usage());
            m.channel = match c {
                "stable" => Channel::Stable, "beta" => Channel::Beta, "edge" => Channel::Edge,
                _ => return Err(format!("bad channel {c}")),
            };
            println!("channel={}", m.channel.label()); Ok(())
        }
        "check" => {
            let n = m.check(1);
            println!("{} updates available on {} channel", n, m.channel.label());
            Ok(())
        }
        "list" => {
            for u in m.for_channel() {
                println!("{:<14} {:<8} {:<6} {} -> {}{}{}", u.id, u.kind.label(), u.channel.label(),
                    u.from_version, u.to_version,
                    if u.requires_reboot { " [reboot]" } else { "" },
                    if u.migration.is_some() { " [migrate]" } else { "" });
            }
            Ok(())
        }
        "apply" => {
            let id = rest.first().map(|s| s.as_str()).unwrap_or_else(|| usage());
            match m.apply(id) {
                Ok(snap) => { println!("applied {id} (snapshot #{snap})"); Ok(()) }
                Err(e) => Err(e.to_string()),
            }
        }
        "apply-all" => {
            let n = m.apply_all();
            println!("applied {n} updates"); Ok(())
        }
        "rollback" => {
            let id = rest.first().map(|s| s.as_str()).unwrap_or_else(|| usage());
            match m.rollback(id) {
                Ok(snap) => { println!("rolled back {id} to snapshot #{snap}"); Ok(()) }
                Err(e) => Err(e.to_string()),
            }
        }
        "reboot-needed" => {
            if m.needs_reboot() { println!("reboot required"); } else { println!("no reboot needed"); }
            Ok(())
        }
        _ => usage(),
    } })();

    if let Err(e) = result {
        eprintln!("omarchy-update: {e}");
        exit(1);
    }
}
