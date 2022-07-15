use crate::*;
use crate::web4helper::*;
use near_sdk::json_types::Base64VecU8;
use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    account_id: Option<AccountId>,
    path: Option<String>,
    params: Option<HashMap<String, String>>,
    query: Option<HashMap<String, Vec<String>>>,
    preloads: Option<HashMap<String, Web4Response>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Response {
    #[serde(rename = "contentType")]
    content_type: Option<String>,
    status: Option<u32>,
    body: Option<Base64VecU8>,
    #[serde(rename = "bodyUrl")]
    body_url: Option<String>,
    #[serde(rename = "preloadUrls")]
    preload_urls: Option<Vec<String>>,
}

impl Web4Response {
    pub fn html_response(text: String) -> Self {
        Self {
            content_type: Some(String::from("text/html; charset=UTF-8")),
            body: Some(text.into_bytes().into()),
            ..Default::default()
        }
    }

    pub fn plain_response(text: String) -> Self {
        Self {
            content_type: Some(String::from("text/plain; charset=UTF-8")),
            body: Some(text.into_bytes().into()),
            ..Default::default()
        }
    }

    pub fn preload_urls(urls: Vec<String>) -> Self {
        Self {
            preload_urls: Some(urls),
            ..Default::default()
        }
    }

    pub fn body_url(url: String) -> Self {
        Self {
            body_url: Some(url),
            ..Default::default()
        }
    }

    pub fn status(status: u32) -> Self {
        Self {
            status: Some(status),
            ..Default::default()
        }
    }
}

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        let path = request.path.expect("ERR_PATH_EXPECTED");

        // Test
        if path == "/test.txt" {
            return Web4Response::status(404);
        }

        // Only NEAR
        // "/processing/verify/NEAR/user.testnet"
        // index.html => verify.html
        if path.starts_with("/processing/verify/NEAR/") {
            let user_id = AccountId::from_str(&path[24..]).expect("ERR_INVALID_ACCOUNT_ID");
            let user_balance = self.get_deposit_by_token(user_id, None).0;
            let decimals = 24;
            let mut user_balance_formatted = yocto_ft(user_balance, decimals).to_string();
            user_balance_formatted = format!("{} NEAR", 
                &user_balance_formatted,
            );

            return Web4Response::html_response(
            include_str!("../res/verify.html")
                .replace("%CONTRACT_ID%", &env::current_account_id().to_string())
                .replace("%BALANCE%", &user_balance_formatted)
            )
        }
        // "/processing/send/NEAR/user.testnet"
        // verify.html => main.html (for send)
        if path.starts_with("/processing/send/NEAR/") {
            let user_id = AccountId::from_str(&path[22..]).expect("ERR_INVALID_ACCOUNT_ID");
            let user_balance = self.get_deposit_by_token(user_id.clone(), None).0;
            let decimals = 24;
            let mut user_balance_formatted = yocto_ft(user_balance, decimals).to_string();
            user_balance_formatted = format!("{} NEAR", 
                &user_balance_formatted,
            );

            let vvault_html = self.get_user_vault_html(user_id.clone());

            return Web4Response::html_response(
            include_str!("../res/main.html")
                .replace("%CONTRACT_ID%", &env::current_account_id().to_string())
                .replace("%BALANCE%", &user_balance_formatted)
                .replace("%USERVAULT%", &vvault_html)
            )
        }

        // Only FT
        // "/processing/verify/ft/token.testnet/user.testnet"
        // index.html => verify.html
        if path.starts_with("/processing/verify/ft/") {
            let parsed_accounts: AccountBalance = parse_user_and_token_ids(path, Web4Pages::Verify);
            let token: AccountId = parsed_accounts.token_id;
            let user_id: AccountId = parsed_accounts.user;
            let user_balance = self.get_deposit_by_token(user_id, Some(token.clone())).0;
            let ticker = self.get_token_ticker(token.clone());
            let decimals = self.get_token_decimals(token);
            let mut user_balance_formatted = yocto_ft(user_balance, decimals).to_string();
            user_balance_formatted = format!("{} {}", 
                &user_balance_formatted,
                &ticker
            );

            return Web4Response::html_response(
            include_str!("../res/verify.html")
                .replace("%CONTRACT_ID%", &env::current_account_id().to_string())
                .replace("%BALANCE%", &user_balance_formatted)
            )
        }

        // "/processing/send/ft/token.testnet/user.testnet"
        // verify.html => main.html (for send)
        if path.starts_with("/processing/send/ft/") {
            let parsed_accounts: AccountBalance = parse_user_and_token_ids(path, Web4Pages::Send);
            let token:AccountId = parsed_accounts.token_id;
            let user_id:AccountId = parsed_accounts.user;
            let user_balance = self.get_deposit_by_token(user_id.clone(), Some(token.clone())).0;
            let ticker = self.get_token_ticker(token.clone());
            let decimals = self.get_token_decimals(token);
            let mut user_balance_formatted = yocto_ft(user_balance, decimals).to_string();
            user_balance_formatted = format!("{} {}", 
                &user_balance_formatted,
                &ticker
            );

            let vvault_html = self.get_user_vault_html(user_id.clone());

            return Web4Response::html_response(
            include_str!("../res/main.html")
                .replace("%CONTRACT_ID%", &env::current_account_id().to_string())
                .replace("%BALANCE%", &user_balance_formatted)
                .replace("%USERVAULT%", &vvault_html)
            )
        }

        // "/processing/finality/user.testnet"
        // after success send to Vault stats
        if path.starts_with("/processing/finality/") {
            let user_id:AccountId = AccountId::from_str(&path[21..]).expect("ERR_PARSE_ACCOUNT_ID");

            let vvault_html = self.get_user_vault_html(user_id.clone());

            return Web4Response::html_response(
            include_str!("../res/main.html")
                .replace("%CONTRACT_ID%", &env::current_account_id().to_string())
                .replace("%BALANCE%", "VAULT MODE")
                .replace("%USERVAULT%", &vvault_html)
            )
        }


        // Index
        let mut app_html = "".to_string();
        let mut select_options="".to_string();
        for (account_id, token_data) in self.get_whitelisted_tokens() {
            // supported tokens to view
            app_html = format!("{}<tr><td><img src=brackets{}brackets/></td><td> {}</td></tr>", 
                &app_html,
                token_data.icon.unwrap_or_default(),
                token_data.ticker.clone().unwrap_or_default(),
            );
            app_html = replace_brackets(app_html);
            // supported tokens to select. HTML:
            // <option value='NEAR:24:NEAR'>NEAR</option>
            // <option value='ft_account.testnet:24:FT'>FT</option>
            select_options = format!("{}<option value='{}:{}:{}''>{}</option>", 
                &select_options,
                account_id.clone(),
                token_data.decimals,
                token_data.ticker.clone().unwrap_or_default(),
                token_data.ticker.unwrap_or_default()
            );
        }

        Web4Response::html_response(
            include_str!("../res/index.html")
                .replace("%TOKENS%", &app_html)
                .replace("%TOKENOPTIONS%", &select_options)
        )
    }

    fn get_user_vault_html(&self, account_id: AccountId) -> String {
        // User Vault
        let mut vault_response = "".to_string();
        let mut vault_token_balances_response = "".to_string();
        let mut vault_used_tokens_response = "".to_string();
        let vvault = self.get_user_vault(account_id);
        for (account, balance) in &vvault.token_deposits {
            vault_token_balances_response = format!("{}{}:{} <i class='nes-icon coin is-small'></i>", 
                &vault_token_balances_response,
                self.get_token_ticker(account.clone()),
                balance.to_string(),
            );
        }
        for token in &vvault.tokens_used {
            vault_used_tokens_response = format!("{}[ {} ]", 
                &vault_used_tokens_response,
                token,
            );
        }
        //<tr><td>{}</td><td>{}</td><td>{}</td><td></td><td>{}</td></tr></td></tr>
        vault_response = format!("{}<tr><td>{}</td><td>{}</td><td>{}</td><td></td><td>{}</td></tr></td></tr>", 
            &vault_response,
            vvault.near_sends_num.to_string(),
            vvault.total_near_amount_sent.to_string(),
            &vault_used_tokens_response,
            &vault_token_balances_response
        );
        vault_response
    }
}