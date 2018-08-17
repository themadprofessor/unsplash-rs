/// Me endpoint.
pub mod me;
/// Photos endpoint.
pub mod photos;

use failure::Fail;
use futures::{Future, Stream};
use hyper::{client::connect::Connect, Client, Method, Request, StatusCode, Uri};
use itertools::Itertools;
use serde::{de::DeserializeOwned, ser::Serialize};

use std::{error::Error as StdError, fmt};

use error::*;

/// A trait to define how to convert a type into a GET Query String.
/// A blanket impl is provided for all Serializable types.
pub trait ToQuery {
    /// Create a GET Query String from self.
    /// If a query string cannot be be create (i.e. self contains no useful
    /// data such as only Nones), then an empty String should be returned.
    /// Otherwise, the returned String must:
    /// - start with a '?'
    /// - be url encoded
    fn to_query(&self) -> String;
}

impl<T> ToQuery for T
where
    T: Serialize,
{
    fn to_query(&self) -> String {
        let s = ::serde_url_params::to_string(&self).unwrap();
        if s.is_empty() {
            String::new()
        } else {
            let mut string = String::with_capacity(s.len() + 1);
            string.push('?');
            string += &s;
            string
        }
    }
}

/// List of errors returned from Unsplash.
/// Unsplash returns a list of Strings upon an error, and this type is used to
/// handle that case. It is normally wrapped in an [Error](struct.Error.html).
#[derive(Debug, Serialize, Deserialize)]
pub struct Errors(Vec<String>);

impl StdError for Errors {}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.0.iter().join("\n"))
    }
}

/// Used to parse JSON into [Errors](struct.Errors.html).
fn parse_err<T>(v: Vec<u8>) -> ::futures::future::FutureResult<T, Error>
where
    T: DeserializeOwned,
{
    match ::serde_json::from_slice::<::endpoint::Errors>(&v) {
        Ok(j) => ::futures::future::err(Error::from(j.context(ErrorKind::MalformedResponse))),
        Err(e) => ::futures::future::err(Error::from(e.context(ErrorKind::MalformedResponse))),
    }
}

/// Used to parse JSON into any serializable type.
fn parse_data<T>(v: Vec<u8>) -> ::futures::future::FutureResult<T, Error>
where
    T: DeserializeOwned,
{
    match ::serde_json::from_slice::<T>(&v) {
        Ok(j) => ::futures::future::ok(j),
        Err(e) => ::futures::future::err(Error::from(e.context(ErrorKind::MalformedResponse))),
    }
}

/// Convenience method for performing a GET request to Unsplash, determining if
/// an error occur and returning a Future to represent this.
///

fn get<T, C, R>(
    query: T,
    client: &Client<C>,
    auth: &str,
    uri: Uri,
) -> impl Future<Item = R, Error = Error>
where
    T: Serialize,
    C: Connect + 'static,
    R: DeserializeOwned,
{
    request(query, client, auth, uri, Method::GET)
}

fn put<T, C, R>(
    query: T,
    client: &Client<C>,
    auth: &str,
    uri: Uri,
) -> impl Future<Item = R, Error = Error>
where
    T: Serialize,
    C: Connect + 'static,
    R: DeserializeOwned,
{
    request(query, client, auth, uri, Method::PUT)
}

fn delete<T, C, R>(
    query: T,
    client: &Client<C>,
    auth: &str,
    uri: Uri,
) -> impl Future<Item = R, Error = Error>
where
    T: Serialize,
    C: Connect + 'static,
    R: DeserializeOwned,
{
    request(query, client, auth, uri, Method::DELETE)
}

fn post<T, C, R>(
    query: T,
    client: &Client<C>,
    auth: &str,
    uri: Uri,
) -> impl Future<Item = R, Error = Error>
where
    T: Serialize,
    C: Connect + 'static,
    R: DeserializeOwned,
{
    request(query, client, auth, uri, Method::POST)
}

fn request<T, C, R>(
    query: T,
    client: &Client<C>,
    auth: &str,
    uri: Uri,
    method: Method,
) -> impl Future<Item = R, Error = Error>
where
    T: Serialize,
    C: Connect + 'static,
    R: DeserializeOwned,
{
    debug!("generating request");
    let request = Request::builder()
        .method(method)
        .uri(format!("{}{}", uri, query.to_query()))
        .header("Accept", "application/json")
        .header("Accept-Version", "v1")
        .header("Authorization", auth)
        .body(::hyper::Body::empty())
        .unwrap();
    trace!("request: {:?}", request);

    client.request(request).map_err(move |e| Error::from(e.context(ErrorKind::Request))).and_then(
        |res| {
            debug!("status code: {}", res.status());
            trace!("response: {:?}", res);
            let parser = if res.status().is_success() { parse_data::<R> } else { parse_err };
            let status = res.status().as_u16();

            res.into_body()
                .map_err(|e| Error::from(e.context(ErrorKind::MalformedResponse)))
                .fold(Vec::new(), fold)
                .and_then(parser)
                .map_err(move |e| {
                    if status == StatusCode::FORBIDDEN.as_u16() {
                        Error::from(e.context(ErrorKind::Forbidden))
                    } else {
                        e
                    }
                })
        },
    )
}

/// Used to convert a Stream of Chunks into a Vec to be used for
/// deserialization.
fn fold(mut v: Vec<u8>, chunk: ::hyper::Chunk) -> ::futures::future::Ok<Vec<u8>, Error> {
    v.extend(&chunk[..]);
    ::futures::future::ok(v)
}
