#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, String, Vec, Address, BytesN, IntoVal, Val, TryFromVal, TryIntoVal};

const EVENT: Symbol = symbol_short!("EVENT");
const TICKETS: Symbol = symbol_short!("TICKETS");

#[derive(Clone)]
pub struct Ticket {
    ticket_id: u32,
    owner: Address,
    event: Event,
    is_valid: bool,
}

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

impl IntoVal<Env, Val> for Event {
    fn into_val(self, env: &Env) -> Val {
        (self.organizer, self.event_title, self.event_date, self.event_location, self.tickets_sold, self.total_tickets, self.ticket_price).into_val(env)
    }
}

impl TryFromVal<Env, Val> for Event {
    type Error = ();

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (organizer, event_title, event_date, event_location, tickets_sold, total_tickets, ticket_price): (Address, String, String, String, u32, u32, u64) = val.try_into_val(env).map_err(|_| ())?;
        Ok(Event {
            organizer,
            event_title,
            event_date,
            event_location,
            tickets_sold,
            total_tickets,
            ticket_price,
        })
    }
}

impl IntoVal<Env, Val> for Ticket {
    fn into_val(self, env: &Env) -> Val {
        (self.ticket_id, self.owner, self.event.into_val(env), self.is_valid).into_val(env)
    }
}

impl TryFromVal<Env, Val> for Ticket {
    type Error = ();

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (ticket_id, owner, event_val, is_valid): (u32, Address, Val, bool) = val.try_into_val(env).map_err(|_| ())?;
        let event: Event = Event::try_from_val(env, &event_val).map_err(|_| ())?;
        Ok(Ticket {
            ticket_id,
            owner,
            event,
            is_valid,
        })
    }
}

#[contract]
pub struct EventContract;

#[contractimpl]
impl EventContract {

    pub fn init(env: Env, event: Event) {
        env.storage().instance().set(&EVENT, &event);
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

    pub fn issue_ticket(env: Env, to: Address) -> Result<u32, &'static str> {
        let mut event = env.storage().instance().get(&EVENT).unwrap().try_into_val(env).unwrap();

        if event.tickets_sold < event.total_tickets {
            event.tickets_sold += 1;
            let ticket_id = event.ticket_sold; // TODO: find a better way to generate ID for ticket.
            let ticket = Ticket {
                ticket_id,
                owner: to.clone(),
                event: event.clone(),
                is_valid: true,
            };

            env.storage().instance().set(&EVENT, &event);
            env.storage().instance().set(&(TICKETS, ticket_id), &ticket);
            Ok(ticket_id)
        } else {
            Err("No more tickets available")
        }
    }

    pub fn buy_ticket(env: Env, buyer: Address) -> Result<u32, &'static str> {
        // TODO: Logic to handle buying a ticket. Is this enough?
        Self::issue_ticket(env, buyer)
    }

    pub fn refund_ticket(env: Env, ticket_id: u32) -> bool {
        let mut env_clone = env.clone();
        let mut ticket: Ticket = env_clone
            .storage()
            .instance()
            .get(&(TICKETS, ticket_id))
            .unwrap()
            .try_into_val(&env)
            .unwrap();
        if ticket.is_valid {
            ticket.is_valid = false;
            env_clone.storage().instance().set(&(TICKETS, ticket_id), &ticket);
            let mut event: Event = env_clone.storage().instance().get(&EVENT).unwrap().try_into_val(&env).unwrap();
            event.tickets_sold -= 1;
            env_clone.storage().instance().set(&EVENT, &event);
            true
        } else {
            false
        }
    }


    pub fn transfer_ticket(env: Env, from: Address, to: Address, ticket_id: u32) -> bool {
        let mut env_clone = env.clone();
        let mut ticket: Ticket = env_clone
            .storage()
            .instance()
            .get(&(TICKETS, ticket_id))
            .unwrap()
            .try_into_val(&env)
            .unwrap();
        if ticket.owner == from && ticket.is_valid {
            ticket.owner = to;
            env_clone.storage().instance().set(&(TICKETS, ticket_id), &ticket);
            true
        } else {
            false
        }
    }

    pub fn validate_ticket(env: Env, ticket_id: u32, owner: Address) -> bool {
        let ticket: Option<Ticket> = env.storage().instance().get(&(TICKETS, ticket_id)).unwrap().try_into_val(env).ok();
        match ticket {
            Some(ticket) => ticket.owner == owner && ticket.is_valid,
            None => false,
        }
    }

    pub fn update_event(env: Env, event_title: Option<String>, event_date: Option<String>, event_location: Option<String>, total_tickets: Option<u32>, ticket_price: Option<u64>) {
        let mut event: Event = env.storage().instance().get(&EVENT).unwrap();

        if let Some(title) = event_title {
            event.event_title = title;
        }
        if let Some(date) = event_date {
            event.event_date = date;
        }
        if let Some(location) = event_location {
            event.event_location = location;
        }
        if let Some(tickets) = total_tickets {
            event.total_tickets = tickets;
        }
        if let Some(price) = ticket_price {
            event.ticket_price = price;
        }
        env.storage().instance().set(&EVENT, &event);
    }

    pub fn cancel_event(env: Env) {
        // Logic to handle event cancellation
        env.storage().instance().remove(&EVENT);
    }
}

#[contract]
pub struct EventToken;

#[contractimpl]
impl EventToken {
    pub fn initialize() {}
}