use std::str::FromStr;
use near_sdk::AccountId;

pub (crate) fn replace_brackets(html: String) -> String {
    let bracket_square = &('"'.to_string());
    html.replace("brackets", bracket_square)
}

#[derive(Debug, PartialEq, Eq)]
pub struct AccountBalance {
    pub user: AccountId,
    pub token_id: AccountId
}

#[derive(Debug, PartialEq, Eq)]
pub enum Web4Pages {
    _Index,
    Verify,
    Send
}
 
pub (crate) fn parse_user_and_token_ids(path: String, web4_page: Web4Pages) -> AccountBalance {
    if web4_page == Web4Pages::Verify {
        let token_account = (&path[22..]).to_string();
        let result:Vec<&str> = token_account.split('/').collect();
        let user = AccountId::from_str(result[1]).expect("ERR_INVALID_ACCOUNT_ID");
        let token_id = AccountId::from_str(result[0]).expect("ERR_INVALID_ACCOUNT_ID");
        AccountBalance {
            user,
            token_id
        }
    } else if web4_page == Web4Pages::Send {
        let token_account = (&path[20..]).to_string();
        let result:Vec<&str> = token_account.split('/').collect();
        let user = AccountId::from_str(result[1]).expect("ERR_INVALID_ACCOUNT_ID");
        let token_id = AccountId::from_str(result[0]).expect("ERR_INVALID_ACCOUNT_ID");
        AccountBalance {
            user,
            token_id
        }
    } else {
        unimplemented!()
    }
}


#[test]
fn test_parse_acc_token() {
    let path = Some("/processing/send/ft/token.near/account.near".to_string()).unwrap();
    let accounts = parse_user_and_token_ids(path, Web4Pages::Send);
    let expected:AccountBalance = AccountBalance { 
        user: AccountId::new_unchecked("account.near".into()), 
        token_id: AccountId::new_unchecked("token.near".into()),
    };
    assert_eq!(accounts, expected);
    let path = Some("/processing/verify/ft/token.near/account.near".to_string()).unwrap();
    let accounts = parse_user_and_token_ids(path, Web4Pages::Verify);
    let expected:AccountBalance = AccountBalance { 
        user: AccountId::new_unchecked("account.near".into()), 
        token_id: AccountId::new_unchecked("token.near".into()),
    };
    assert_eq!(accounts, expected);
}