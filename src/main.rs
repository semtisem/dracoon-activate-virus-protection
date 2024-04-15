use async_recursion::async_recursion;
use client::Client;
use dco3::auth::Connected;
use dco3::nodes::{ Node, NodesFilter, RoomPoliciesRequest};
use dco3::{Dracoon, DracoonClientError, ListAllParams, Nodes, Rooms};
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

    let dracoon = Client::connect_auth_code_flow(config.get_dracoon_config().get_base_url(), CLIENT_ID.to_owned(), CLIENT_SECRET.to_owned()).await;
    let mut collected_rooms = Vec::new();

    for room_id in config.get_virus_protection_rooms() {
        if *room_id == 0 {
            error!("Room id is 0. Activating virus protection for every room is not allowed.");
            continue;
        }

        let current_room = dracoon.nodes.get_node(*room_id).await.unwrap(); 
        collected_rooms.push(current_room);
        let temp_vec = &mut Vec::new();
        let rooms =  get_all_room_children(&dracoon, *room_id, temp_vec).await;
        match rooms {
            Ok(rooms) => {
                collected_rooms.extend(rooms.iter().cloned());
            }
            Err(e) =>  {
                error!("Error while fetching rooms for room with id: {}. Error: {}", room_id, e);
            }
            
        }
    }

    info!("Collected room names: {:?}", collected_rooms.iter().map(|room| room.name.clone()).collect::<Vec<String>>());
    activate_virus_protection(&dracoon, collected_rooms).await;

    
}


#[async_recursion]
async fn get_all_room_children<'a>(dracoon: &'a Dracoon<Connected>, room_id: u64, collected_room_ids: &'a mut Vec<Node>) -> Result<&'a mut Vec<Node>, DracoonClientError> {
    let params = ListAllParams::builder()
        .with_filter(NodesFilter::is_room())
        .build();

    let rooms_res = dracoon.nodes.get_nodes(Some(room_id), None, Some(params)).await.unwrap();
    let mut room_list = rooms_res.items.clone();

    if rooms_res.items.len() > 500 {
        let mut offset = 500;

        while offset < rooms_res.range.total {
            let params_with_offset = ListAllParams::builder()
                .with_filter(NodesFilter::is_room())
                .with_offset(offset)
                .build();

            let rooms_res_offset = dracoon.nodes.get_nodes(Some(room_id), None, Some(params_with_offset)).await;
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
    for room in room_list.iter(){
        if let Some(cnt_rooms) = room.cnt_rooms {
            if cnt_rooms > 0 {
                get_all_room_children(dracoon, room.id, collected_room_ids).await?;
            }
        }
    }
    Ok(collected_room_ids)
}

async fn activate_virus_protection(dracoon: &Dracoon<Connected>, rooms: Vec<Node>) {
    for room in rooms {
        let policies = RoomPoliciesRequest::builder().with_virus_protection_enabled(true).build();
        let response = dracoon.nodes.update_room_policies(room.id,policies).await;
        match response {
            Ok(_) => info!("Successfully activated virus protection for room '{}' with id: {}", room.name,room.id),
            Err(e) => error!("Failed to activate virus protection for room '{}' with id: {}. Error: {}", room.name, room.id, e)
        }
    }
}
