use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct RecyclableItem {
    id: u64,                      // unique identifier
    category: String,            // e.g., plastic, paper
    description: String,         // item details
    estimated_price: u64,        // suggested price in EGP
    image_url: String,           // URL to the image
    matched: bool,               // true if a business accepted it
    timestamp: u64,              // when it was submitted
}

thread_local! {
    static ITEMS: RefCell<HashMap<u64, RecyclableItem>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(1);  // auto-increment ID
}

#[update]
fn list_recyclable_item(
    category: String,
    description: String,
    estimated_price: u64,
    image_url: String,
) -> u64 {
    let id = NEXT_ID.with(|counter| {
        let mut c = counter.borrow_mut();
        let current = *c;
        *c += 1;
        current
    });

    let item = RecyclableItem {
        id,
        category,
        description,
        estimated_price,
        image_url,
        matched: false,
        timestamp: time(),
    };

    ITEMS.with(|items| {
        items.borrow_mut().insert(id, item);
    });

    id
}

#[update]
fn match_with_business(item_id: u64) -> Result<String, String> {
    ITEMS.with(|items| {
        let mut items = items.borrow_mut();
        match items.get_mut(&item_id) {
            Some(item) => {
                if item.matched {
                    Err("Item already matched".to_string())
                } else {
                    item.matched = true;
                    Ok("Item successfully matched with a business!".to_string())
                }
            }
            None => Err("Item not found.".to_string()),
        }
    })
}

#[query]
fn get_all_items() -> Vec<RecyclableItem> {
    ITEMS.with(|items| items.borrow().values().cloned().collect())
}

