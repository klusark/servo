/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::global::{GlobalRef, GlobalField};
use dom::bindings::js::Root;
use dom::bindings::utils::{Reflector, reflect_dom_object};
use dom::bindings::error::Fallible;
use dom::bindings::codegen::Bindings::HistoryBinding;
use dom::bindings::codegen::Bindings::HistoryBinding::HistoryMethods;
use js::jsapi::{JSContext, HandleValue};

use util::str::DOMString;

use num::ToPrimitive;
use std::borrow::ToOwned;

#[dom_struct]
pub struct History {
    length: Reflector
}

impl History {
    pub fn new_inherited(global: GlobalRef) -> History {
        History {
            length: Reflector::new()
        }
    }

    pub fn new(global: GlobalRef) -> Root<History> {
        reflect_dom_object(box History::new_inherited(global), global, HistoryBinding::Wrap)
    }

}

impl<'a> HistoryMethods for &'a History {
    // http://dev.w3.org/2006/webapi/FileAPI/#dfn-size
    fn Length(self) -> i32{
       0// self.length
    }



    // http://dev.w3.org/2006/webapi/FileAPI/#slice-method-algo
    fn Go(self, delta: Option<i32>) {

    }

    fn Back(self) {
    }

    fn Forward(self) {
    }

    fn PushState(self, cx: *mut JSContext, data: HandleValue, title: DOMString, url: Option<DOMString>) {
    }

    fn ReplaceState(self, cx: *mut JSContext, data: HandleValue, title: DOMString, url: Option<DOMString>) {
    }

}

