extern crate dotenv;

use std::env;
use std::time::Instant;
use dotenv::dotenv;
use matrix_sdk::{
	config::SyncSettings, room::Room,
	ruma::events::room::{
		member::StrippedRoomMemberEvent,
		message::{
			OriginalSyncRoomMessageEvent, TextMessageEventContent, MessageType, RoomMessageEventContent
		}
	}, Client,
};
use tokio::time::{sleep, Duration};
use log::{info, debug};
use regex::Regex;
use lazy_static::lazy_static;
use async_std_resolver::resolver_from_system_conf;

async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
	lazy_static! {
		static ref RE: Regex = Regex::new("resolve ([a-z\\._0-9]+\\.[a-z]+)").unwrap();
	}

	if let Room::Joined(room) = room {
		let mut msg_body = match event.content.msgtype {
			MessageType::Text(TextMessageEventContent { body, .. }) => body,
			_ => return,
		};

		msg_body = msg_body.to_lowercase();

		if msg_body.contains("resolve help") {
			let content = RoomMessageEventContent::text_html(
				"Just send 'resolve (domain)'. 'resolve source' to get the code.",
				"Just send <code>resolve (domain)</code>. <code>resolve source</code> to get the code.");
			room.send(content, None).await.unwrap();
		} else if msg_body.contains("resolve source") {
			let content = RoomMessageEventContent::text_plain("https://git.sr.ht/~cofob/matrix-dnsbot");
			room.send(content, None).await.unwrap();
		} else if msg_body.contains("resolve") {
			let caps = RE.captures(&msg_body);
			match caps {
				Some(cap) => {
					let start = Instant::now();
					let domain = cap.get(1).unwrap().as_str();
					let resolver = resolver_from_system_conf().await.unwrap();
					let lookup = resolver.lookup_ip(domain).await;
					if lookup.is_ok() {
						let lookup = lookup.unwrap();
						if lookup.iter().count() == 0 {
							let content = RoomMessageEventContent::text_plain("empty");
							room.send(content, None).await.unwrap();
							return;
						}

						let mut plain_answer = String::from("IP record");
						let mut answer = String::from("IP record");

						if lookup.iter().count() > 1 {
							// lets goooo shitcode
							plain_answer.push_str("s for ");
							plain_answer.push_str(domain);
							plain_answer.push_str(":");
							answer.push_str("s for <code>");
							answer.push_str(domain);
							answer.push_str("</code>:");
						} else {
							plain_answer.push_str(" for ");
							plain_answer.push_str(domain);
							plain_answer.push_str(":");
							answer.push_str(" for <code>");
							answer.push_str(domain);
							answer.push_str("</code>:");
						}

						for ip in lookup {
							plain_answer.push_str(" '");
							plain_answer.push_str(&ip.to_string());
							plain_answer.push_str("';");
							answer.push_str(" <code>");
							answer.push_str(&ip.to_string());
							answer.push_str("</code>;");
						}
						let end = Instant::now();
						let delta = end - start;
						plain_answer.push_str(" That took ");
						plain_answer.push_str(&delta.as_millis().to_string());
						plain_answer.push_str(" ms.");
						answer.push_str(" That took <code>");
						answer.push_str(&delta.as_millis().to_string());
						answer.push_str("</code> ms.");
						let content = RoomMessageEventContent::text_html(plain_answer, answer);
						room.send(content, None).await.unwrap();
					} else {
						let content = RoomMessageEventContent::text_plain("Unexpected error occured. Most likely, the domain simply does not have IP records.");
						room.send(content, None).await.unwrap();
					}
				}
    		_ => debug!("regex not found"),
			}
		} else if room.is_direct() {
			let content = RoomMessageEventContent::text_plain("I dont understand you! Send `resolve help` to get help.");
			room.send(content, None).await.unwrap();
		}
	}
}

async fn on_stripped_state_member(
	room_member: StrippedRoomMemberEvent,
	client: Client,
	room: Room,
) {
	if room_member.state_key != client.user_id().await.unwrap() {
		return;
	}

	if let Room::Invited(room) = room {
		info!("Autojoining room {}", room.room_id());
		let mut delay = 2;

		while let Err(err) = room.accept_invitation().await {
			info!("Failed to join room {} ({:?}), retrying in {}s", room.room_id(), err, delay);

			sleep(Duration::from_secs(delay)).await;
			delay *= 2;

			if delay > 3600 {
				info!("Can't join room {} ({:?})", room.room_id(), err);
				break;
			}
		}
		info!("Successfully joined room {}", room.room_id());
	}
}

async fn login_and_sync(
	homeserver_url: String,
	username: &str,
	password: &str,
) -> anyhow::Result<()> {
	#[allow(unused_mut)]
	let mut client_builder = Client::builder().homeserver_url(homeserver_url);

	let state_store = matrix_sdk_sled::StateStore::open_with_path(env::var("DB")
		.unwrap_or_else(|_| "./db/".to_string()))?;
	let crypto_store = matrix_sdk_sled::CryptoStore::open_with_passphrase(env::var("DB")
		.unwrap_or_else(|_| "./db/".to_string()), Some("passphrase"))?;

	client_builder = client_builder.state_store(Box::new(state_store)).crypto_store(Box::new(crypto_store));

	let client = client_builder.build().await?;

	let response = client.login(username, password, Some("bot"), Some(&env::var("SESSION_NAME")
		.unwrap_or_else(|_| "bot".to_string()))).await?;

	info!("logged in as {}", response.user_id);

	client.register_event_handler(on_stripped_state_member).await;
	client.register_event_handler(on_room_message).await;

	client.sync(SyncSettings::default()).await;

	Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();

	dotenv().ok();

	let (homeserver_url, username, password) = (
		env::var("HOMESERVER").expect("HOMESERVER variable is"),
		env::var("USERNAME").expect("USERNAME variable is"),
		env::var("PASSWORD").expect("PASSWORD variable is"));

	login_and_sync(homeserver_url, &username, &password).await?;
	Ok(())
}
