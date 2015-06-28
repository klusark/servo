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

#[jstraceable]
struct State {
    //data: Heap<HandleValue>,
    title: DOMString,
    url: Option<DOMString>
}

#[dom_struct]
pub struct History {
    state: Vec<State>,
    current: i32,
}

impl History {
    pub fn new_inherited(global: GlobalRef) -> History {
        History {
            state: Vec::new(),
            current: 0
        }
    }

    pub fn new(global: GlobalRef) -> Root<History> {
        reflect_dom_object(box History::new_inherited(global), global, HistoryBinding::Wrap)
    }

}

impl<'a> HistoryMethods for &'a History {
    // http://dev.w3.org/2006/webapi/FileAPI/#dfn-size
    fn Length(self) -> i32 {
       0// self.length
    }

    fn Go(self, delta: Option<i32>) {

    }

    fn Back(self) {
        self.Go(Some(-1));
    }

    fn Forward(self) {
        self.Go(Some(1));
    }

    fn PushState(self, cx: *mut JSContext, data: HandleValue, title: DOMString, url: Option<DOMString>) {
        self.state.push(State { /*data: data,*/ title: title, url: url } );
    }

    fn ReplaceState(self, cx: *mut JSContext, data: HandleValue, title: DOMString, url: Option<DOMString>) {
    }

}

