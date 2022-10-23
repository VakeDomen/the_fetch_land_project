
use std::error::Error;

use crate::{models::{subscription::Subscription, user::User, card::Card}, api::notification_telegram};

use super::{database::user_operations::get_user_by_id, card_cache::ALL_CARDS, mail::send_mail, telegram::notify_via_telegram};

pub async fn notify_subscription(sub: Subscription) -> Result<(), Box<dyn Error>> {
    let sqlite_user_option = match get_user_by_id(sub.user_id.clone()) {
        Ok(sqlite_user_option) => sqlite_user_option,
        Err(e) => return Err(e.into()),
    };
    let user = match sqlite_user_option {
        Some(sqlite_user) => User::from(sqlite_user),
        None => return Err(format!("User for subscription does not seem to exist: {}", sub.user_id).into()),
    };
    let (card_name, card_id) = {
        let all_cards = ALL_CARDS.lock().unwrap();
        let card = all_cards.get(&sub.sale_object_id.to_string());
        match card {
            Some(c) => (c.name.clone(), c.id.clone()),
            None => return Err(format!("Card ID does not seem to exist: {}", sub.sale_object_id).into()),
        }
    };
    let subject = build_email_subject(card_name.clone());
    let body = build_email_body( card_name, card_id);

    match send_mail(
        &user.email.as_str(), 
        subject.as_str(), 
        body
    ){
        Ok(_) => Ok(()),
        Err(e) => {
            match notify_via_telegram(
                format!("Error sending subscription email to {}", user.email), 
                "[SYSTEM]".to_string()
            ).await {
                Ok(_) => Err(e.into()),
                Err(_) => Err(e.into()),
            }
        },
    }
}

fn build_email_subject(c_name: String) -> String {
    format!("Nova ponudba karte: {}", c_name)
}

fn build_email_body(c_name: String, c_id: String) -> String {
    format!("Pozdravljeni,\nobveščamo vas, da je nekdo objavil novo ponudbo za karto {}.\nOglase karte lahko najdete na strani:\n\thttps://fetchland.eu/search?id={}", c_name, c_id)
}