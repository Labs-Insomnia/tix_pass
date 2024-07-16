#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, String, Vec, Address, BytesN};

#[derive(Clone)]
pub struct Event {
    organizer: Address,
    event_title: String,
    event_date: String,
    event_location: String,
    tickets_sold: u32,
    total_tickets: u32,
    ticket_price: u64
}

const EVENT: Symbol = symbol_short!("EVENT");

#[contract]
pub struct EventContract;

#[contractimpl]
impl EventContract {

    pub fn init(env: Env, event: Event) {

    }

    pub fn create_event(env: Env, organizer: Address, total_tickets: u32, ticket_price: u64, event_title: String, event_date: String, event_location: String) {
        let event = Event {
            organizer,
            event_title,
            event_date,
            event_location,
            tickets_sold: 0,
            total_tickets,
            ticket_price
        };

        // save the event.
        env.storage().instance().set(&EVENT, &event);
    }

    pub fn issue_ticket() {}

    pub fn buy_ticket() {}

    pub fn refund_ticket() {}

    pub fn transfer_ticket() {}

    pub fn validate_ticket() {}

    pub fn update_event() {}

    pub fn cancel_event() {}
}

#[contract]
pub struct EventToken;

#[contractimpl]
impl EventToken {
    pub fn initialize() {}
}