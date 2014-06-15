// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::hashmap::HashMap;
use std::gc::Gc;
use std::slice::Items;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;

#[deriving(Show)]
pub enum AttributeValue {
  UIntValue(uint),
  StrValue(String),
  RefValue(String),
}

pub enum AttributeType {
  StringAttribute,
  IntAttribute,
  RefAttribute,
}

#[deriving(Show)]
pub struct Attribute {
  pub value: AttributeValue,
  pub key_span: Span,
  pub value_span: Span,
}

impl Attribute {
  pub fn new(value: AttributeValue, key_span: Span, value_span: Span)
      -> Attribute {
    Attribute {
      value: value,
      key_span: key_span,
      value_span: value_span,
    }
  }
}

#[deriving(Show)]
pub struct Node {
  pub name: Option<String>,
  pub name_span: Span,

  pub path: String,
  pub path_span: Span,

  pub attributes: HashMap<String, Attribute>,

  pub subnodes: Vec<Gc<Node>>,
}

impl Node {
  pub fn new(name: Option<String>, name_span: Span, path: String,
      path_span: Span) -> Node {
    Node {
      name: name,
      name_span: name_span,
      path: path,
      path_span: path_span,
      attributes: HashMap::new(),
      subnodes: Vec::new(),
    }
  }

  pub fn get_attr<'a>(&'a self, key: &str) -> &'a Attribute {
    self.attributes.get(&key.to_str())
  }

  pub fn get_string_attr<'a>(&'a self, key: &str) -> Option<&'a String> {
    self.attributes.find(&key.to_str()).and_then(|av| match av.value {
      StrValue(ref s) => Some(s),
      _ => None,
    })
  }

  pub fn get_int_attr(&self, key: &str) -> Option<uint> {
    self.attributes.find(&key.to_str()).and_then(|av| match av.value {
      UIntValue(ref u) => Some(*u),
      _ => None,
    })
  }

  pub fn get_ref_attr<'a>(&'a self, key: &str) -> Option<&'a String> {
    self.attributes.find(&key.to_str()).and_then(|av| match av.value {
      RefValue(ref s) => Some(s),
      _ => None,
    })
  }

  pub fn get_required_string_attr<'a>(&'a self, cx: &ExtCtxt, key: &str)
      -> Option<&'a String> {
    match self.get_string_attr(key) {
      Some(val) => Some(val),
      None => {
        cx.parse_sess().span_diagnostic.span_err(self.name_span,
            format!("required string attribute `{}` is missing", key)
            .as_slice());
        None
      }
    }
  }

  pub fn get_required_int_attr<'a>(&'a self, cx: &ExtCtxt, key: &str)
      -> Option<uint> {
    match self.get_int_attr(key) {
      Some(val) => Some(val),
      None => {
        cx.parse_sess().span_diagnostic.span_err(self.name_span,
            format!("required integer attribute `{}` is missing", key)
            .as_slice());
        None
      }
    }
  }

  pub fn get_required_ref_attr<'a>(&'a self, cx: &ExtCtxt, key: &str)
      -> Option<&'a String> {
    match self.get_ref_attr(key) {
      Some(val) => Some(val),
      None => {
        cx.parse_sess().span_diagnostic.span_err(self.name_span,
            format!("required ref attribute `{}` is missing", key)
            .as_slice());
        None
      }
    }
  }

  pub fn expect_no_attributes(&self, cx: &ExtCtxt) -> bool {
    let mut ok = true;
    for (_, v) in self.attributes.iter() {
      ok = false;
      cx.parse_sess().span_diagnostic.span_err(v.key_span,
          "no attributes expected");
    }
    ok
  }

  pub fn expect_no_subnodes(&self, cx: &ExtCtxt) -> bool {
    let mut ok = true;
    for sub in self.subnodes.iter() {
      ok = false;
      cx.parse_sess().span_diagnostic.span_err(sub.name_span,
          "no subnodes expected");
    }
    ok
  }

  pub fn expect_attributes(&self, cx: &ExtCtxt,
      expectations: Vec<(&str, AttributeType)>) -> bool {
    let mut ok = true;
    for &(n, ref t) in expectations.iter() {
      match t {
        &StringAttribute => {
          if self.get_required_string_attr(cx, n).is_none() {ok = false}
        },
        &IntAttribute => {
          if self.get_required_int_attr(cx, n).is_none() {ok = false}
        },
        &RefAttribute => {
          if self.get_required_ref_attr(cx, n).is_none() {ok = false}
        },
      }
    }
    ok
  }

  pub fn get_by_path<'a>(&'a self, path: &str) -> Option<&'a Gc<Node>> {
    // TODO(farcaller): if this is commonly used it would be better to rewrite
    // subnodes as hash as well.
    let path_str = path.to_str();
    for n in self.subnodes.iter() {
      if n.path == path_str {
        return Some(n);
      }
    }
    None
  }
}

#[deriving(Show)]
pub struct PlatformTree {
  nodes: Vec<Gc<Node>>,
  named: HashMap<String, Gc<Node>>,
}

impl PlatformTree {
  pub fn new(nodes: Vec<Gc<Node>>, named: HashMap<String, Gc<Node>>)
      -> PlatformTree {
    PlatformTree {
      nodes: nodes,
      named: named,
    }
  }

  pub fn get_by_name<'a>(&'a self, name: &str) -> Option<&'a Gc<Node>> {
    self.named.find(&name.to_str())
  }

  pub fn get<'a>(&'a self, idx: uint) -> &'a Gc<Node> {
    self.nodes.get(idx)
  }

  pub fn iter<'a>(&'a self) -> Items<'a, Gc<Node>> {
    self.nodes.iter()
  }
}
