use std::time::Duration;

use async_recursion::async_recursion;
use client::Client;
use console::Term;
use dco3::auth::Connected;
use dco3::nodes::{Node, NodesFilter, RoomPoliciesRequest};
use dco3::{Dracoon, DracoonClientError, ListAllParams, Nodes, Rooms};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tracing::{error, info};

use crate::config::ScriptConfig;
use crate::logging::Logging;

mod client;
mod config;
mod logging;

#[tokio::main]
async fn main() {
    const CLIENT_ID: &str = "Q8dTruVvswW5Iyi0QWqiZFKP8gFgFjnZ";
    const CLIENT_SECRET: &str = "nhh27DqlmFEjf5ijVAN0ZoBdhKMDu4lv";

    let config = ScriptConfig::init(None);
    Logging::setup(config.get_logging_config());
    let term = Term::stdout();

    let dracoon = Client::connect_auth_code_flow(
        config.get_dracoon_config().get_base_url(),
        CLIENT_ID.to_owned(),
        CLIENT_SECRET.to_owned(),
    )
    .await;
    let mut collected_rooms = Vec::new();

    let multibar = MultiProgress::new();

    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    bar.enable_steady_tick(Duration::from_millis(120));

    multibar.add(bar.clone());

    for room_id in config.get_virus_protection_rooms() {
        if *room_id == 0 {
            error!("Room id is 0. Activating virus protection for every room is not allowed.");
            continue;
        }
        bar.set_message(std::format!(
            "Processing room with id: {}. Fetching all sub rooms",
            room_id
        ));

        let current_room = dracoon.nodes.get_node(*room_id).await.unwrap();
        collected_rooms.push(current_room);
        let temp_vec = &mut Vec::new();

        let bar_sub_rooms = ProgressBar::new_spinner();
        bar_sub_rooms.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        bar_sub_rooms.enable_steady_tick(Duration::from_millis(120));

        multibar.add(bar_sub_rooms.clone());
        let rooms =
            get_all_room_children(&dracoon, *room_id, temp_vec, bar_sub_rooms.clone()).await;
        match rooms {
            Ok(rooms) => {
                collected_rooms.extend(rooms.iter().cloned());
            }
            Err(e) => {
                error!(
                    "Error while fetching rooms for room with id: {}. Error: {}",
                    room_id, e
                );
            }
        }
        multibar.remove(&bar_sub_rooms);
    }
    bar.finish_with_message(format!("Collected {} rooms", collected_rooms.len()));
    info!(
        "Collected room names: {:?}",
        collected_rooms
            .iter()
            .map(|room| room.name.clone())
            .collect::<Vec<String>>()
    );

    let total_size = collected_rooms.len();
    let progress_bar = ProgressBar::new(total_size as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    progress_bar.set_message("Activating virus protection for rooms...");

    activate_virus_protection(
        &dracoon,
        collected_rooms,
        term.clone(),
        progress_bar.clone(),
    )
    .await;

    progress_bar.finish_with_message("Finished activating virus protection for rooms");
}

#[async_recursion]
async fn get_all_room_children<'a>(
    dracoon: &'a Dracoon<Connected>,
    room_id: u64,
    collected_room_ids: &'a mut Vec<Node>,
    sub_bar: ProgressBar,
) -> Result<&'a mut Vec<Node>, DracoonClientError> {
    let params = ListAllParams::builder()
        .with_filter(NodesFilter::is_room())
        .build();

    sub_bar.set_message(std::format!(
        "Fetching sub rooms for room with id: {}",
        room_id
    ));

    let rooms_res = dracoon
        .nodes
        .get_nodes(Some(room_id), None, Some(params))
        .await
        .unwrap();
    let mut room_list = rooms_res.items.clone();

    if rooms_res.items.len() > 500 {
        let mut offset = 500;

        while offset < rooms_res.range.total {
            let params_with_offset = ListAllParams::builder()
                .with_filter(NodesFilter::is_room())
                .with_offset(offset)
                .build();

            let rooms_res_offset = dracoon
                .nodes
                .get_nodes(Some(room_id), None, Some(params_with_offset))
                .await;
            if let Ok(rooms_res_offset) = rooms_res_offset {
                room_list.extend(rooms_res_offset.items);
            } else {
                error!("Error while fetching rooms with offset: {:?}", offset);
                break;
            }
            offset += 500;
        }
    }

    collected_room_ids.extend(room_list.iter().cloned());
    for room in room_list.iter() {
        if let Some(cnt_rooms) = room.cnt_rooms {
            if cnt_rooms > 0 {
                get_all_room_children(dracoon, room.id, collected_room_ids, sub_bar.clone())
                    .await?;
            }
        }
    }
    Ok(collected_room_ids)
}

async fn activate_virus_protection(
    dracoon: &Dracoon<Connected>,
    rooms: Vec<Node>,
    term: Term,
    progress_bar: ProgressBar,
) {
    for room in rooms {
        let progress_bar = progress_bar.clone();
        let policies = RoomPoliciesRequest::builder()
            .with_virus_protection_enabled(true)
            .build();
        let response = dracoon.nodes.update_room_policies(room.id, policies).await;
        match response {
            Ok(_) => {
                info!(
                    "Successfully activated virus protection for room '{}' with id: {}",
                    room.name, room.id
                );
                progress_bar.set_message(std::format!(
                    "Successfully activated virus protection for room '{}' with id: {}",
                    room.name,
                    room.id
                ));
                progress_bar.inc(1);
            }

            Err(e) => {
                error!(
                    "Failed to activate virus protection for room '{}' with id: {}. Error: {}",
                    room.name, room.id, e
                );
                term.write_line(
                    std::format!(
                        "Failed to activate virus protection for room '{}' with id: {}. Error: {}",
                        room.name,
                        room.id,
                        e
                    )
                    .as_str(),
                )
                .unwrap();
            }
        }
    }
}
