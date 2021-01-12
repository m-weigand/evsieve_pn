// SPDX-License-Identifier: GPL-2.0-or-later

use std::fmt;
use crate::domain::Domain;
use crate::ecodes;

pub type EventType = u16;
pub type EventCode = u16;
pub type EventId = (EventType, EventCode);
pub type EventValue = i32;


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Event {
    pub ev_type: EventType,
    pub code: EventCode,
    pub value: EventValue,

    /// The value this event had the last time it was emitted by a device.
    pub previous_value: EventValue,

    pub domain: Domain,
    pub namespace: Namespace,
}

impl Event {
    pub fn new(ev_type: EventType,
               code: EventCode,
               value: EventValue,
               previous_value: EventValue,
               domain: Domain,
               namespace: Namespace
    ) -> Event {
        Event { ev_type, code, value, previous_value, domain, namespace }
    }

    pub fn with_domain(mut self, new_domain: Domain) -> Event {
        self.domain = new_domain;
        self
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = ecodes::event_name(self.ev_type, self.code);
        write!(f, "{}:{}", name, self.value)
    }
}


/// Namespaces are an internal concept that is not visible to the user. They are like domains, but
/// then on a higher level such that even a filter with an empty domain cannot match events within a
/// different namespace.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Namespace {
    /// This event was generated by an input device and has not yet entered the processing stream
    /// from the end user's perspective. It is not affected by any `StreamEntry` except `StreamEntry::Input`.
    Input,
    /// This event is in the processing stream.
    User,
    /// This event was generated by --map yield or similar. It is not affected by any `StreamEntry`
    /// except for `StreamEntry::Output`.
    Yielded,
    /// This event was caught by an --output and shall now be sent to an output device. It is not
    /// affected by any StreamEntry.
    Output,
}