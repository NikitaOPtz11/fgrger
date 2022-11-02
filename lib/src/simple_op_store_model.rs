// Autogenerated by Thrift Compiler (0.17.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::vec_box)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use thrift::OrderedFloat;
use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSerializable, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;

//
// RefConflict
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RefConflict {
  pub removes: Vec<Vec<u8>>,
  pub adds: Vec<Vec<u8>>,
}

impl RefConflict {
  pub fn new(removes: Vec<Vec<u8>>, adds: Vec<Vec<u8>>) -> RefConflict {
    RefConflict {
      removes,
      adds,
    }
  }
}

impl TSerializable for RefConflict {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<RefConflict> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<Vec<u8>>> = None;
    let mut f_2: Option<Vec<Vec<u8>>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Vec<u8>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = i_prot.read_bytes()?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Vec<u8>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_1 = i_prot.read_bytes()?;
            val.push(list_elem_1);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("RefConflict.removes", &f_1)?;
    verify_required_field_exists("RefConflict.adds", &f_2)?;
    let ret = RefConflict {
      removes: f_1.expect("auto-generated code should have checked for presence of required fields"),
      adds: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("RefConflict");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("removes", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::String, self.removes.len() as i32))?;
    for e in &self.removes {
      o_prot.write_bytes(e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("adds", TType::List, 2))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::String, self.adds.len() as i32))?;
    for e in &self.adds {
      o_prot.write_bytes(e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// RefTarget
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RefTarget {
  CommitId(Vec<u8>),
  Conflict(RefConflict),
}

impl TSerializable for RefTarget {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<RefTarget> {
    let mut ret: Option<RefTarget> = None;
    let mut received_field_count = 0;
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_bytes()?;
          if ret.is_none() {
            ret = Some(RefTarget::CommitId(val));
          }
          received_field_count += 1;
        },
        2 => {
          let val = RefConflict::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(RefTarget::Conflict(val));
          }
          received_field_count += 1;
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
          received_field_count += 1;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    if received_field_count == 0 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received empty union from remote RefTarget"
          )
        )
      )
    } else if received_field_count > 1 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received multiple fields for union from remote RefTarget"
          )
        )
      )
    } else {
      Ok(ret.expect("return value should have been constructed"))
    }
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("RefTarget");
    o_prot.write_struct_begin(&struct_ident)?;
    match *self {
      RefTarget::CommitId(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("commit_id", TType::String, 1))?;
        o_prot.write_bytes(f)?;
        o_prot.write_field_end()?;
      },
      RefTarget::Conflict(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("conflict", TType::Struct, 2))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// RemoteBranch
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RemoteBranch {
  pub remote_name: String,
  pub target: RefTarget,
}

impl RemoteBranch {
  pub fn new(remote_name: String, target: RefTarget) -> RemoteBranch {
    RemoteBranch {
      remote_name,
      target,
    }
  }
}

impl TSerializable for RemoteBranch {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<RemoteBranch> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = None;
    let mut f_2: Option<RefTarget> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = RefTarget::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("RemoteBranch.remote_name", &f_1)?;
    verify_required_field_exists("RemoteBranch.target", &f_2)?;
    let ret = RemoteBranch {
      remote_name: f_1.expect("auto-generated code should have checked for presence of required fields"),
      target: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("RemoteBranch");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("remote_name", TType::String, 1))?;
    o_prot.write_string(&self.remote_name)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("target", TType::Struct, 2))?;
    self.target.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// Branch
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Branch {
  pub name: String,
  pub local_target: Option<RefTarget>,
  pub remote_branches: Vec<RemoteBranch>,
}

impl Branch {
  pub fn new<F2>(name: String, local_target: F2, remote_branches: Vec<RemoteBranch>) -> Branch where F2: Into<Option<RefTarget>> {
    Branch {
      name,
      local_target: local_target.into(),
      remote_branches,
    }
  }
}

impl TSerializable for Branch {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Branch> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = None;
    let mut f_2: Option<RefTarget> = None;
    let mut f_3: Option<Vec<RemoteBranch>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = RefTarget::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        3 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<RemoteBranch> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_2 = RemoteBranch::read_from_in_protocol(i_prot)?;
            val.push(list_elem_2);
          }
          i_prot.read_list_end()?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Branch.name", &f_1)?;
    verify_required_field_exists("Branch.remote_branches", &f_3)?;
    let ret = Branch {
      name: f_1.expect("auto-generated code should have checked for presence of required fields"),
      local_target: f_2,
      remote_branches: f_3.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Branch");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("name", TType::String, 1))?;
    o_prot.write_string(&self.name)?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.local_target {
      o_prot.write_field_begin(&TFieldIdentifier::new("local_target", TType::Struct, 2))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_begin(&TFieldIdentifier::new("remote_branches", TType::List, 3))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Struct, self.remote_branches.len() as i32))?;
    for e in &self.remote_branches {
      e.write_to_out_protocol(o_prot)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// GitRef
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GitRef {
  pub name: String,
  pub target: RefTarget,
}

impl GitRef {
  pub fn new(name: String, target: RefTarget) -> GitRef {
    GitRef {
      name,
      target,
    }
  }
}

impl TSerializable for GitRef {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<GitRef> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = None;
    let mut f_2: Option<RefTarget> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = RefTarget::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("GitRef.name", &f_1)?;
    verify_required_field_exists("GitRef.target", &f_2)?;
    let ret = GitRef {
      name: f_1.expect("auto-generated code should have checked for presence of required fields"),
      target: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("GitRef");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("name", TType::String, 1))?;
    o_prot.write_string(&self.name)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("target", TType::Struct, 2))?;
    self.target.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// Tag
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tag {
  pub name: String,
  pub target: RefTarget,
}

impl Tag {
  pub fn new(name: String, target: RefTarget) -> Tag {
    Tag {
      name,
      target,
    }
  }
}

impl TSerializable for Tag {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Tag> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = None;
    let mut f_2: Option<RefTarget> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = RefTarget::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Tag.name", &f_1)?;
    verify_required_field_exists("Tag.target", &f_2)?;
    let ret = Tag {
      name: f_1.expect("auto-generated code should have checked for presence of required fields"),
      target: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Tag");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("name", TType::String, 1))?;
    o_prot.write_string(&self.name)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("target", TType::Struct, 2))?;
    self.target.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// View
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct View {
  pub head_ids: Vec<Vec<u8>>,
  pub public_head_ids: Vec<Vec<u8>>,
  pub wc_commit_ids: BTreeMap<String, Vec<u8>>,
  pub branches: Vec<Branch>,
  pub tags: Vec<Tag>,
  pub git_refs: Vec<GitRef>,
  pub git_head: Option<Vec<u8>>,
}

impl View {
  pub fn new<F7>(head_ids: Vec<Vec<u8>>, public_head_ids: Vec<Vec<u8>>, wc_commit_ids: BTreeMap<String, Vec<u8>>, branches: Vec<Branch>, tags: Vec<Tag>, git_refs: Vec<GitRef>, git_head: F7) -> View where F7: Into<Option<Vec<u8>>> {
    View {
      head_ids,
      public_head_ids,
      wc_commit_ids,
      branches,
      tags,
      git_refs,
      git_head: git_head.into(),
    }
  }
}

impl TSerializable for View {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<View> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<Vec<u8>>> = None;
    let mut f_2: Option<Vec<Vec<u8>>> = None;
    let mut f_3: Option<BTreeMap<String, Vec<u8>>> = None;
    let mut f_4: Option<Vec<Branch>> = None;
    let mut f_5: Option<Vec<Tag>> = None;
    let mut f_6: Option<Vec<GitRef>> = None;
    let mut f_7: Option<Vec<u8>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Vec<u8>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_3 = i_prot.read_bytes()?;
            val.push(list_elem_3);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Vec<u8>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_4 = i_prot.read_bytes()?;
            val.push(list_elem_4);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        3 => {
          let map_ident = i_prot.read_map_begin()?;
          let mut val: BTreeMap<String, Vec<u8>> = BTreeMap::new();
          for _ in 0..map_ident.size {
            let map_key_5 = i_prot.read_string()?;
            let map_val_6 = i_prot.read_bytes()?;
            val.insert(map_key_5, map_val_6);
          }
          i_prot.read_map_end()?;
          f_3 = Some(val);
        },
        4 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Branch> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_7 = Branch::read_from_in_protocol(i_prot)?;
            val.push(list_elem_7);
          }
          i_prot.read_list_end()?;
          f_4 = Some(val);
        },
        5 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Tag> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_8 = Tag::read_from_in_protocol(i_prot)?;
            val.push(list_elem_8);
          }
          i_prot.read_list_end()?;
          f_5 = Some(val);
        },
        6 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<GitRef> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_9 = GitRef::read_from_in_protocol(i_prot)?;
            val.push(list_elem_9);
          }
          i_prot.read_list_end()?;
          f_6 = Some(val);
        },
        7 => {
          let val = i_prot.read_bytes()?;
          f_7 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("View.head_ids", &f_1)?;
    verify_required_field_exists("View.public_head_ids", &f_2)?;
    verify_required_field_exists("View.wc_commit_ids", &f_3)?;
    verify_required_field_exists("View.branches", &f_4)?;
    verify_required_field_exists("View.tags", &f_5)?;
    verify_required_field_exists("View.git_refs", &f_6)?;
    let ret = View {
      head_ids: f_1.expect("auto-generated code should have checked for presence of required fields"),
      public_head_ids: f_2.expect("auto-generated code should have checked for presence of required fields"),
      wc_commit_ids: f_3.expect("auto-generated code should have checked for presence of required fields"),
      branches: f_4.expect("auto-generated code should have checked for presence of required fields"),
      tags: f_5.expect("auto-generated code should have checked for presence of required fields"),
      git_refs: f_6.expect("auto-generated code should have checked for presence of required fields"),
      git_head: f_7,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("View");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("head_ids", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::String, self.head_ids.len() as i32))?;
    for e in &self.head_ids {
      o_prot.write_bytes(e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("public_head_ids", TType::List, 2))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::String, self.public_head_ids.len() as i32))?;
    for e in &self.public_head_ids {
      o_prot.write_bytes(e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("wc_commit_ids", TType::Map, 3))?;
    o_prot.write_map_begin(&TMapIdentifier::new(TType::String, TType::String, self.wc_commit_ids.len() as i32))?;
    for (k, v) in &self.wc_commit_ids {
      o_prot.write_string(k)?;
      o_prot.write_bytes(v)?;
    }
    o_prot.write_map_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("branches", TType::List, 4))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Struct, self.branches.len() as i32))?;
    for e in &self.branches {
      e.write_to_out_protocol(o_prot)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("tags", TType::List, 5))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Struct, self.tags.len() as i32))?;
    for e in &self.tags {
      e.write_to_out_protocol(o_prot)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("git_refs", TType::List, 6))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Struct, self.git_refs.len() as i32))?;
    for e in &self.git_refs {
      e.write_to_out_protocol(o_prot)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.git_head {
      o_prot.write_field_begin(&TFieldIdentifier::new("git_head", TType::String, 7))?;
      o_prot.write_bytes(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// Operation
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Operation {
  pub view_id: Vec<u8>,
  pub parents: Vec<Vec<u8>>,
  pub metadata: Box<OperationMetadata>,
}

impl Operation {
  pub fn new(view_id: Vec<u8>, parents: Vec<Vec<u8>>, metadata: Box<OperationMetadata>) -> Operation {
    Operation {
      view_id,
      parents,
      metadata,
    }
  }
}

impl TSerializable for Operation {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Operation> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<u8>> = None;
    let mut f_2: Option<Vec<Vec<u8>>> = None;
    let mut f_3: Option<Box<OperationMetadata>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_bytes()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Vec<u8>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_10 = i_prot.read_bytes()?;
            val.push(list_elem_10);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        3 => {
          let val = Box::new(OperationMetadata::read_from_in_protocol(i_prot)?);
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Operation.view_id", &f_1)?;
    verify_required_field_exists("Operation.parents", &f_2)?;
    verify_required_field_exists("Operation.metadata", &f_3)?;
    let ret = Operation {
      view_id: f_1.expect("auto-generated code should have checked for presence of required fields"),
      parents: f_2.expect("auto-generated code should have checked for presence of required fields"),
      metadata: f_3.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Operation");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("view_id", TType::String, 1))?;
    o_prot.write_bytes(&self.view_id)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("parents", TType::List, 2))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::String, self.parents.len() as i32))?;
    for e in &self.parents {
      o_prot.write_bytes(e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("metadata", TType::Struct, 3))?;
    self.metadata.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// Timestamp
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timestamp {
  pub millis_since_epoch: i64,
  pub tz_offset: i32,
}

impl Timestamp {
  pub fn new(millis_since_epoch: i64, tz_offset: i32) -> Timestamp {
    Timestamp {
      millis_since_epoch,
      tz_offset,
    }
  }
}

impl TSerializable for Timestamp {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Timestamp> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i64> = None;
    let mut f_2: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i64()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_i32()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Timestamp.millis_since_epoch", &f_1)?;
    verify_required_field_exists("Timestamp.tz_offset", &f_2)?;
    let ret = Timestamp {
      millis_since_epoch: f_1.expect("auto-generated code should have checked for presence of required fields"),
      tz_offset: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Timestamp");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("millis_since_epoch", TType::I64, 1))?;
    o_prot.write_i64(self.millis_since_epoch)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("tz_offset", TType::I32, 2))?;
    o_prot.write_i32(self.tz_offset)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// OperationMetadata
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OperationMetadata {
  pub start_time: Timestamp,
  pub end_time: Timestamp,
  pub description: String,
  pub hostname: String,
  pub username: String,
  pub tags: BTreeMap<String, String>,
}

impl OperationMetadata {
  pub fn new(start_time: Timestamp, end_time: Timestamp, description: String, hostname: String, username: String, tags: BTreeMap<String, String>) -> OperationMetadata {
    OperationMetadata {
      start_time,
      end_time,
      description,
      hostname,
      username,
      tags,
    }
  }
}

impl TSerializable for OperationMetadata {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<OperationMetadata> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Timestamp> = None;
    let mut f_2: Option<Timestamp> = None;
    let mut f_3: Option<String> = None;
    let mut f_4: Option<String> = None;
    let mut f_5: Option<String> = None;
    let mut f_6: Option<BTreeMap<String, String>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = Timestamp::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let val = Timestamp::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_string()?;
          f_3 = Some(val);
        },
        4 => {
          let val = i_prot.read_string()?;
          f_4 = Some(val);
        },
        5 => {
          let val = i_prot.read_string()?;
          f_5 = Some(val);
        },
        6 => {
          let map_ident = i_prot.read_map_begin()?;
          let mut val: BTreeMap<String, String> = BTreeMap::new();
          for _ in 0..map_ident.size {
            let map_key_11 = i_prot.read_string()?;
            let map_val_12 = i_prot.read_string()?;
            val.insert(map_key_11, map_val_12);
          }
          i_prot.read_map_end()?;
          f_6 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("OperationMetadata.start_time", &f_1)?;
    verify_required_field_exists("OperationMetadata.end_time", &f_2)?;
    verify_required_field_exists("OperationMetadata.description", &f_3)?;
    verify_required_field_exists("OperationMetadata.hostname", &f_4)?;
    verify_required_field_exists("OperationMetadata.username", &f_5)?;
    verify_required_field_exists("OperationMetadata.tags", &f_6)?;
    let ret = OperationMetadata {
      start_time: f_1.expect("auto-generated code should have checked for presence of required fields"),
      end_time: f_2.expect("auto-generated code should have checked for presence of required fields"),
      description: f_3.expect("auto-generated code should have checked for presence of required fields"),
      hostname: f_4.expect("auto-generated code should have checked for presence of required fields"),
      username: f_5.expect("auto-generated code should have checked for presence of required fields"),
      tags: f_6.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("OperationMetadata");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("start_time", TType::Struct, 1))?;
    self.start_time.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("end_time", TType::Struct, 2))?;
    self.end_time.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("description", TType::String, 3))?;
    o_prot.write_string(&self.description)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("hostname", TType::String, 4))?;
    o_prot.write_string(&self.hostname)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("username", TType::String, 5))?;
    o_prot.write_string(&self.username)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("tags", TType::Map, 6))?;
    o_prot.write_map_begin(&TMapIdentifier::new(TType::String, TType::String, self.tags.len() as i32))?;
    for (k, v) in &self.tags {
      o_prot.write_string(k)?;
      o_prot.write_string(v)?;
    }
    o_prot.write_map_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

