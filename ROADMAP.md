# Tix Pass: Stellar Event Ticketing System

# FLOW
1. Ticket Issuance
- issue digital tickets as Stellar assets
- each tickets will be represented as a unique token
2. Ticket Purchase
- users purchase tickets by sending Stellar lumens (XLM) to an account
- after, a ticket token is transferred to the user's Stellar account

## Features
Ticket creation/issuance: don't know how that will work yet.
Ticket transfer: to allow users to transfer tickets to other accounts
Ticket validation: event organizers can validate the tickets using an identifier or a token. so we check the validity and ownership.

## Challenge
During validation, how do we account for situations where someone gives their ticket to another?

## Development
TicketContract
- issue_ticket
- transfer_ticket
- validate_ticket



---
each event has its own ticket characteristics, contracts, etc.
SQL table for all events, with a tickets for each event.
organizers pay for creating events, and set the conditions (amount of tickets, cost of tickets, date, etc)
there is a base template for each event written as code. we use the conditions provided by organizer, fill it as
params in our template, compile it, and we deploy it as a contract/event. the contract will know the number of tickets, 
and will issue the token (ticket)  to each wallet who wants to attend the event.
