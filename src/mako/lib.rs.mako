<% 
	from util import (iter_nested_types, new_context, rust_comment, rust_doc_comment,
                      rust_module_doc_comment, rb_type, hub_type, mangle_ident)
	nested_schemas = list(iter_nested_types(schemas))

 	c = new_context(resources)

	hub_type = hub_type(canonicalName)
    
%>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="lib/util.mako"/>\
<%namespace name="rbuild" file="lib/rbuild.mako"/>\
<%namespace name="mbuild" file="lib/mbuild.mako"/>\
<%namespace name="schema" file="lib/schema.mako"/>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>

<%block filter="rust_module_doc_comment">\
${lib.docs(c)}
</%block>
#![feature(core)]

extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

mod cmn;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::default::Default;

pub use cmn::{Hub, ResourceMethodsBuilder, MethodBuilder, Resource, Part, ResponseResult, RequestValue, NestedType};

// ########
// HUB ###
// ######

/// Central instance to access all ${hub_type} related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
<%block filter="rust_doc_comment">\
<%lib:hub_usage_example/>\
</%block>
pub struct ${hub_type}<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for ${hub_type}<C, NC, A> {}

impl<'a, C, NC, A> ${hub_type}<C, NC, A>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> + 'a,
            A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> ${hub_type}<C, NC, A> {
        ${hub_type} {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _m: PhantomData,
        }
    }

    % for resource in sorted(c.rta_map.keys()):
    pub fn ${mangle_ident(resource)}(&'a self) -> ${rb_type(resource)}<'a, C, NC, A> {
        ${rb_type(resource)} { hub: &self }
    }
    % endfor
}


// ############
// SCHEMAS ###
// ##########
% for s in schemas.values():
${schema.new(s, c)}
% endfor

// ###################
// NESTED SCHEMAS ###
// #################
## some schemas are only used once and basically internal types.
## We have to find them and process them as normal types
% for s in nested_schemas:
${schema.new(s, c)}
% endfor

// ###################
// MethodBuilders ###
// #################

% for resource in c.rta_map:
${rbuild.new(resource, c)}


% endfor


// ###################
// CallBuilders   ###
// #################

% for resource, methods in c.rta_map.iteritems():
% for method in methods:
${mbuild.new(resource, method, c)}

% endfor ## method in methods
% endfor ## resource, methods