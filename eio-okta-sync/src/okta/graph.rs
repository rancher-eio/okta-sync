use std::collections::BTreeMap;

use eio_okta_data::current::management::components::schemas::{User, UserProfile};
use itertools::Itertools;
use petgraph::prelude::*;

use crate::okta::UserProfileExtensions;

pub(crate) struct Org<'users> {
  pub(crate) hierarchy: DiGraphMap<&'users str, &'users str>,
  user_id_cache: BTreeMap<&'users str, &'users User>,
  pub(crate) users: &'users [User],
}

impl<'users> Org<'users> {
  pub(crate) fn new(users: &'users [User]) -> Self {
    Self {
      hierarchy: Default::default(),
      user_id_cache: Default::default(),
      users,
    }
  }

  pub(crate) fn user(&self, id: &str) -> Option<&&User> {
    self.user_id_cache.get(id)
  }

  #[allow(dead_code)]
  pub(crate) fn above(&self, user: &'users str) -> Vec<&'users str> {
    self
      .hierarchy
      .neighbors_directed(&user, Direction::Incoming)
      .fold(Vec::new(), |mut management, manager| {
        management.extend(self.above(manager));
        management.push(manager);
        management
      })
  }

  pub(crate) fn below(&self, user: &'users str) -> Vec<&'users str> {
    self
      .hierarchy
      .neighbors_directed(user, Direction::Outgoing)
      .fold(Vec::new(), |mut employees, employee| {
        employees.push(employee);
        employees.extend(self.below(employee));
        employees
      })
  }

  pub(crate) fn populate_naive(mut self) -> Self {
    for user in self.users {
      self.user_id_cache.insert(&user.id, user);

      match user.manager(self.users) {
        Some(manager) => {
          self.hierarchy.add_edge(&manager.id, &user.id, Default::default());
        }
        None => {
          self.hierarchy.add_node(&user.id);
        }
      }
    }

    self
  }

  pub(crate) fn populate_descending(mut self) -> Self {
    for user in self.users.iter().filter(|user| user.is_own_manager()) {
      self.add_descending(user);
    }

    self
  }

  fn add_descending(&mut self, user: &'users User) -> &'users str {
    let node_a = self.hierarchy.add_node(user.id.as_str());

    self.user_id_cache.insert(node_a, user);

    for direct_report in user.direct_reports(self.users) {
      let node_b = self.add_descending(direct_report);
      self.hierarchy.add_edge(node_a, node_b, Default::default());
    }

    node_a
  }

  pub(crate) fn populate_ascending(mut self) -> Self {
    let users = self
      .users
      .iter()
      .filter(|user| user.direct_reports(&self.users).is_empty())
      .collect_vec();

    for user in users {
      self.add_ascending(user);
    }

    self
  }

  fn add_ascending(&mut self, user: &'users User) -> &'users str {
    let node_b = self.hierarchy.add_node(user.id.as_str());

    self.user_id_cache.insert(node_b, user);

    if !user.is_own_manager()
      && let Some(manager) = user.manager(&self.users)
    {
      let node_a = self.add_ascending(manager);
      self.hierarchy.add_edge(node_a, node_b, Default::default());
    }

    node_b
  }
}

pub(crate) trait Management: AsRef<User> {
  fn is_managed_by_user(&self, user: impl AsRef<User>) -> bool {
    let this = self.as_ref();
    let that = user.as_ref();

    match &this.profile {
      UserProfile {
        manager_id: Some(manager_id),
        ..
      } => that
        .profile
        .employee_number
        .as_ref()
        .is_some_and(|employee_number| manager_id.eq(employee_number)),
      UserProfile {
        manager_id: None,
        manager: Some(manager),
        ..
      } => that
        .profile
        .extensions_into::<UserProfileExtensions>()
        .is_ok_and(|extensions| {
          extensions
            .domain_id
            .is_some_and(|domain_id| manager.eq_ignore_ascii_case(&domain_id))
        }),
      _ => false,
    }
  }

  fn is_own_manager(&self) -> bool {
    self.is_managed_by_user(self)
  }

  fn manager<'users>(&self, users: &'users [User]) -> Option<&'users User> {
    if self.is_own_manager() {
      return None;
    }

    users.iter().find(|user| self.is_managed_by_user(user))
  }

  fn direct_reports<'users>(&self, users: &'users [User]) -> Vec<&'users User> {
    users
      .iter()
      .filter(|user| user.is_managed_by_user(self))
      .filter(|user| !user.is_own_manager())
      .collect()
  }
}

impl Management for User {}
