// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

mod access_control;
mod diamond;
mod flashloan;
mod ownable;
mod pausable;
mod payment_splitter;
mod psp22;
mod psp34;
mod psp35;
mod reentrancy_guard;
mod timelock_controller;

pub use access_control::AccessControlError;
pub use diamond::DiamondError;
pub use flashloan::{
    FlashBorrowerError,
    FlashLenderError,
};
pub use ownable::OwnableError;
pub use pausable::PausableError;
pub use payment_splitter::PaymentSplitterError;
pub use psp22::{
    PSP22Error,
    PSP22ReceiverError,
    PSP22TokenTimelockError,
};
pub use psp34::{
    PSP34Error,
    PSP34ReceiverError,
};
pub use psp35::{
    PSP35Error,
    PSP35ReceiverError,
};
pub use reentrancy_guard::ReentrancyGuardError;
pub use timelock_controller::TimelockControllerError;
