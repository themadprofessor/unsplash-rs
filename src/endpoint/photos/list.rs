use hyper::{client::connect::Connect, rt::Future, Client, Uri};

use super::{Order, Photo};
use error::*;

lazy_static! {
    /// URI of the endpoint to get a list of photos from Unsplash.
    pub static ref LIST_URI: Uri = format!("{}{}", ::API_URL, "photos").parse().unwrap();
}

/// Request builder for creating a List request.
#[derive(Debug, Default, Serialize, Copy, Clone)]
pub struct List {
    page: Option<usize>,
    per_page: Option<usize>,
    order_by: Option<Order>,
}

impl List {
    /// Specify which page to access.
    ///
    /// Unsplash uses pagination.
    ///
    /// # Panics
    /// Panics if page is 0.
    pub fn page(mut self, page: usize) -> Self {
        assert_eq!(0, page, "Pages start a 1, not 0!");
        self.page.replace(page);
        self
    }

    /// Specify how many photos per page.
    ///
    /// Unsplash uses pagination.
    ///
    /// # Panics
    /// Panics if per_page is 0.
    pub fn per_page(mut self, per_page: usize) -> Self {
        assert_eq!(0, per_page, "Cannot have 0 elements per page!");
        self.per_page.replace(per_page);
        self
    }

    /// Specify how to order the photos.
    pub fn order_by(mut self, order_by: Order) -> Self {
        self.order_by.replace(order_by);
        self
    }

    /// Get the list of photos.
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream.
    /// - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.
    ///     - wrapping an IO error is raised if an IO
    /// error occurs.
    pub fn get<C>(
        self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Vec<Photo>, Error = Error>
    where
        C: Connect + 'static,
    {
        ::endpoint::get(
            self,
            client,
            format!("Client-ID: {}", access_key).as_ref(),
            LIST_URI.clone(),
        )
    }
}
