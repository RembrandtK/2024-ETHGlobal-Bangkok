use idkit::session::AppId;

use std::str::FromStr;

use crate::{action::core::verify_user_action, args::Args};

const CANDIDIATE_VOTE_ACTION: &str = "vote-for-candidate";
const ACTION_SIGNAL: &str = "";

pub async fn approve_vote(args: Args) -> eyre::Result<()> {
    let app_id = AppId::from_str(&args.app_id)?;

    verify_user_action(
        app_id,
        CANDIDIATE_VOTE_ACTION,
        ACTION_SIGNAL,
        Some(&args.candidate),
    )
    .await
}
