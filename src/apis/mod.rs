/// this module contains Admin apis
/// account CRUD
/// And should contain virtual platform management
pub mod admin;

/// this module contains Account apis
/// which allow the user to modify his email and password
pub mod account;

/// this module contains Authentication related functions which are
/// Http Post Login
/// Http Get  Refresh
/// Security wrapping function
pub mod authentication;

pub mod cfd;

/// this module contains apis that allow the ViceDoyen to:
/// Manage sessions
/// Manage applicants
/// Make announcements
pub mod vice_doyen;
