//! This module handles rendering parsed data to a window

use std::{collections::HashMap, sync::mpsc::Receiver};

use eframe::{egui, epi};

use crate::ipc::{MdnsPacket, Service};

/// Defines a single host record
#[derive(Debug)]
struct HostRecord {
    pub name: String,
    pub services: HashMap<String, Vec<Service>>,
}

/// This is the GUI, and everything it needs to keep state
pub struct BrowseApp {
    /// The data stream coming from the subprocess
    packet_stream: Receiver<MdnsPacket>,

    /// The currently rendered hosts
    current_hosts: HashMap<String, HashMap<String, HostRecord>>,
}

impl BrowseApp {
    pub fn new(packet_stream: Receiver<MdnsPacket>) -> Self {
        Self {
            packet_stream,
            current_hosts: HashMap::new(),
        }
    }
}

impl epi::App for BrowseApp {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        // Poll the packet stream for new data, and update the hosts list accordingly
        for packet in self.packet_stream.try_iter() {
            match packet.mode {
                crate::ipc::PacketMode::New => {
                    // Create the interface if not already existing
                    if !self.current_hosts.contains_key(&packet.interface_name) {
                        self.current_hosts
                            .insert(packet.interface_name.clone(), HashMap::new());
                    }

                    // Add the host to the interface
                    self.current_hosts
                        .get_mut(&packet.interface_name)
                        .unwrap()
                        .insert(
                            packet.hostname.clone(),
                            HostRecord {
                                name: packet.hostname.clone(),
                                services: match packet.service {
                                    Some(service) => {
                                        let mut services = HashMap::new();
                                        services.insert(service.name.clone(), vec![service]);
                                        services
                                    }
                                    None => HashMap::new(),
                                },
                            },
                        );
                }
                crate::ipc::PacketMode::Update => {
                    // Get the interface
                    let interface = self.current_hosts.get_mut(&packet.interface_name).unwrap();

                    // Get the host
                    let host = interface.get_mut(&packet.hostname).unwrap();

                    // Only update the host if the new service is real
                    if let Some(service) = packet.service {
                        // If the service name does not exist, create the service on the host
                        if !host.services.contains_key(&service.name) {
                            host.services.insert(service.name.clone(), vec![service]);
                        } else {
                            // Update an existing service if the address is not already used
                            let services = host.services.get_mut(&service.name).unwrap();
                            if !services.iter().any(|s| {
                                format!("{} {}:{}", s.hostname, s.ip, s.port)
                                    == format!(
                                        "{} {}:{}",
                                        service.hostname, service.ip, service.port
                                    )
                            }) {
                                services.push(service);
                            }
                        }
                    }
                }
                crate::ipc::PacketMode::Remove => {
                    // TODO: I don't want to actually remove data from the view. Maybe this can get implemented in the future?
                }
            }
        }

        // Create a menu bar with quit button
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        // Create a scrollable area to display the packet data
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Split the GUI into interfaces
                for (interface, hosts) in &self.current_hosts {
                    // Create a header for the interface
                    ui.add(egui::Label::new(interface).heading());
                    ui.separator();

                    // Render each host
                    for host in hosts.values() {
                        // Render the packet data
                        egui::CollapsingHeader::new(&host.name).show(ui, |ui| {
                            // Render every service name as a dropdown
                            for service_name in host.services.keys() {
                                egui::CollapsingHeader::new(service_name).show(ui, |ui| {
                                    // Render provider for the service as its own dropdown, containing optional metadata
                                    for service in host.services.get(service_name).unwrap() {
                                        egui::CollapsingHeader::new(format!(
                                            "{} (IP: {} Port: {})",
                                            service.hostname, service.ip, service.port
                                        ))
                                        .show(ui, |ui| {
                                            // Render the metadata
                                            ui.label(service.data.join("\n"));
                                        });
                                    }
                                });
                            }
                        });
                    }
                }
            });
        });
    }

    fn name(&self) -> &str {
        "NetBrowse"
    }
}
