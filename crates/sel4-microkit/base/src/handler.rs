//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use core::fmt;

use crate::{
    defer::{DeferredAction, PreparedDeferredAction},
    ipc::{self, ChannelSet, Event},
    pd_is_passive, Channel, Child, MessageInfo,
};

pub use core::convert::Infallible;

/// Trait for the application-specific part of a protection domain's main loop.
pub trait Handler {
    /// Error type returned by this protection domain's entrypoints.
    type Error: fmt::Display;

    /// This method has the same meaning and type as its analog in `libmicrokit`.
    ///
    /// The default implementation just panics.
    fn notified(&mut self, channels: ChannelSet) -> Result<(), Self::Error> {
        panic!(
            "unexpected notification from channels {}",
            channels.display()
        )
    }

    /// This method has the same meaning and type as its analog in `libmicrokit`.
    ///
    /// The default implementation just panics.
    fn protected(
        &mut self,
        channel: Channel,
        msg_info: MessageInfo,
    ) -> Result<MessageInfo, Self::Error> {
        panic!("unexpected protected procedure call from channel {channel:?} with msg_info={msg_info:?}")
    }

    /// This method has the same meaning and type as its analog in `libmicrokit`.
    ///
    /// The default implementation just panics.
    fn fault(
        &mut self,
        child: Child,
        msg_info: MessageInfo,
    ) -> Result<Option<MessageInfo>, Self::Error> {
        panic!("unexpected fault from protection domain {child:?} with msg_info={msg_info:?}")
    }

    /// An advanced feature for use by protection domains which seek to coalesce syscalls when
    /// possible.
    ///
    /// This method is used by the main loop to fuse a queued `seL4_Send` call with the next
    /// `seL4_Recv` using `seL4_NBSendRecv`. Its default implementation just returns `None`.
    fn take_deferred_action(&mut self) -> Option<DeferredAction> {
        None
    }

    #[doc(hidden)]
    fn run(&mut self) -> Result<Never, Self::Error> {
        let mut reply_tag: Option<MessageInfo> = None;

        let mut prepared_deferred_action: Option<PreparedDeferredAction> = if pd_is_passive() {
            Some(ipc::forfeit_sc())
        } else {
            None
        };

        loop {
            let event = match (reply_tag.take(), prepared_deferred_action.take()) {
                (Some(msg_info), None) => ipc::reply_recv(msg_info),
                (None, Some(action)) => ipc::nb_send_recv(action),
                (None, None) => ipc::recv(),
                _ => panic!("handler yielded deferred action after call to 'protected()'"),
            };

            match event {
                Event::Notified(channels) => {
                    self.notified(channels)?;
                }
                Event::Protected(channel, msg_info) => {
                    reply_tag = Some(self.protected(channel, msg_info)?);
                }
                Event::Fault(child, msg_info) => {
                    reply_tag = self.fault(child, msg_info)?;
                }
            };

            prepared_deferred_action = self
                .take_deferred_action()
                .as_ref()
                .map(DeferredAction::prepare);
        }
    }
}

#[doc(hidden)]
pub enum Never {}

impl Handler for Never {
    type Error = Infallible;
}

/// A [`Handler`] implementation which does not override any of the default method implementations.
pub struct NullHandler(());

impl NullHandler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(())
    }
}

impl Handler for NullHandler {
    type Error = Infallible;
}
