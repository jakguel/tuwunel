use axum::extract::State;
use futures::{FutureExt, StreamExt, pin_mut};
use ruma::{
	api::client::user_directory::search_users::{self},
	events::room::join_rules::JoinRule,
};
use tuwunel_core::{
	Result,
	utils::{future::BoolExt, stream::BroadbandExt},
};

use crate::Ruma;

// Tuwunel can handle a lot more results than synapse
const LIMIT_MAX: usize = 500;
const LIMIT_DEFAULT: usize = 10;

/// # `POST /_matrix/client/r0/user_directory/search`
///
/// Searches all known users for a match.
///
/// - Hides any local users that aren't in any public rooms (i.e. those that
///   have the join rule set to public) and don't share a room with the sender
pub(crate) async fn search_users_route(
	State(services): State<crate::State>,
	body: Ruma<search_users::v3::Request>,
) -> Result<search_users::v3::Response> {
	let sender_user = body.sender_user();
	let limit = usize::try_from(body.limit)
		.map_or(LIMIT_DEFAULT, usize::from)
		.min(LIMIT_MAX);

	let search_term = body.search_term.to_lowercase();
	let mut users = services
		.users
		.stream()
		.map(ToOwned::to_owned)
		.broad_filter_map(async |user_id| {
			let display_name = services.users.displayname(&user_id).await.ok();

			let user_id_matches = user_id
				.as_str()
				.to_lowercase()
				.contains(&search_term);

			let display_name_matches = display_name
				.as_deref()
				.map(str::to_lowercase)
				.is_some_and(|display_name| display_name.contains(&search_term));

			if !user_id_matches && !display_name_matches {
				return None;
			}

			let user_in_public_room = services
				.rooms
				.state_cache
				.rooms_joined(&user_id)
				.map(ToOwned::to_owned)
				.broad_any(async |room_id| {
					services
						.rooms
						.state_accessor
						.get_join_rules(&room_id)
						.map(|rule| matches!(rule, JoinRule::Public))
						.await
				});

			let user_sees_user = services
				.rooms
				.state_cache
				.user_sees_user(sender_user, &user_id);

			pin_mut!(user_in_public_room, user_sees_user);
			user_in_public_room
				.or(user_sees_user)
				.await
				.then_some(search_users::v3::User {
					user_id: user_id.clone(),
					display_name,
					avatar_url: services.users.avatar_url(&user_id).await.ok(),
				})
		});

	let results = users.by_ref().take(limit).collect().await;
	let limited = users.next().await.is_some();

	Ok(search_users::v3::Response { results, limited })
}
