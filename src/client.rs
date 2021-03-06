/*  Copyright 2015 MaidSafe.net limited
    This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
    version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
    licence you accepted on initial access to the Software (the "Licences").
    By contributing code to the MaidSafe Software, or to this project generally, you agree to be
    bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
    directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
    available at: http://www.maidsafe.net/licenses
    Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
    under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
    OF ANY KIND, either express or implied.
    See the Licences for the specific language governing permissions and limitations relating to
    use of the MaidSafe
    Software.                                                                 */

#![allow(unused_variables)]

extern crate lru_cache;

extern crate routing;
extern crate maidsafe_types;

use std::sync::{Mutex, Arc, Condvar};
use self::lru_cache::LruCache;

use self::routing::Action;
use self::routing::RoutingError;
use self::routing::types::Authority;
use self::routing::types::DestinationAddress;
use self::routing::types::DhtId;


pub struct ClientFacade {
  my_cvar : Arc<(Mutex<bool>, Condvar)>,
  my_cache : LruCache<u32, Result<Vec<u8>, RoutingError>>
}

impl routing::Facade for ClientFacade {
  fn handle_get(&mut self, type_id: u64, our_authority: Authority, from_authority: Authority,
                from_address: DhtId, name: DhtId)->Result<Action, RoutingError> {
    ;
    Err(RoutingError::InvalidRequest)
  }

  fn handle_put(&mut self, our_authority: Authority, from_authority: Authority,
                from_address: DhtId, dest_address: DestinationAddress, data: Vec<u8>)->Result<Action, RoutingError> {
    ;
    Err(RoutingError::InvalidRequest)
  }

  fn handle_post(&mut self, our_authority: Authority, from_authority: Authority, from_address: DhtId, data: Vec<u8>)->Result<Action, RoutingError> {
    ;
    Err(RoutingError::InvalidRequest)
  }

  fn handle_get_response(&mut self, from_address: DhtId, response: Result<Vec<u8>, RoutingError>) {
    // TODO message_id needs to be passed in here
    self.my_cache.insert(0, response);
    let &(ref lock, ref cvar) = &*self.my_cvar;
    let mut fetched = lock.lock().unwrap();
    *fetched = true;
    cvar.notify_one();
  }

  fn handle_put_response(&mut self, from_authority: Authority, from_address: DhtId, response: Result<Vec<u8>, RoutingError>) {
    ;
  }

  fn handle_post_response(&mut self, from_authority: Authority, from_address: DhtId, response: Result<Vec<u8>, RoutingError>) {
    ;
  }

  fn add_node(&mut self, node: DhtId) { unimplemented!() }

  fn drop_node(&mut self, node: DhtId) { unimplemented!() }
}

impl ClientFacade {
  pub fn new(cvar: Arc<(Mutex<bool>, Condvar)>) -> ClientFacade {
    ClientFacade { my_cvar: cvar, my_cache: LruCache::new(10000) }
  }

  pub fn get_response(&mut self, message_id : u32) -> Result<Vec<u8>, RoutingError> {
    let result = self.my_cache.remove(&message_id).unwrap();
    result
  }
}
