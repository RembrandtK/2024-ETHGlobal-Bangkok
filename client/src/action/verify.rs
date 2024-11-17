use idkit::session::AppId;

use std::str::FromStr;

use crate::{action::core::verify_user_action, args::Args};

const TEST_ACTION: &str = "test-action";
const ACTION_SIGNAL: &str = "";

pub async fn verify_user(args: Args) -> eyre::Result<()> {
    let app_id = AppId::from_str("app_ce4cb73cb75fc3b73b71ffb4de178410")?;

    verify_user_action(
        app_id,
        TEST_ACTION,
        ACTION_SIGNAL,
        Some(&args.candidate),
    )
    .await
}
