use crate::MapInto;
use crate::Options;
use eio_okta_api::authorization::SSWS;
use eio_okta_api::traits::Endpoint;
use eio_okta_api::traits::IntoRequest;
use eio_okta_api::traits::Pagination;
use eio_okta_api::traits::Response;
use eio_okta_api::traits::Service;
use headers::Authorization;
use headers::HeaderMapExt;
use http::Request;
use http::Uri;

impl From<Options> for Client<crate::OktaService> {
  fn from(options: Options) -> Self {
    let Options {
      agent,
      authorization,
      auto_paginate,
      service,
    } = options;

    Self {
      agent: agent.into(),
      authorization,
      auto_paginate,
      service,
    }
  }
}

pub struct Client<S>
where
  S: Service,
{
  pub agent: ureq::Agent,
  pub authorization: Authorization<SSWS>,
  pub auto_paginate: bool,
  pub service: S,
}

use crate::Error;

impl<S> Client<S>
where
  S: Service,
{
  pub fn uri<T>(&self, endpoint: &T) -> Result<Uri, Error>
  where
    T: Endpoint,
  {
    let base = self.service.uri()?;
    match base.authority_str() {
      Some(authority) => Uri::builder().authority(authority),
      None => Uri::builder(),
    }
    .scheme(base.scheme_str())
    .path_and_query(endpoint.uri()?.as_str())
    .build()
    .map_into()
  }

  pub fn request<T>(&self, endpoint: T) -> Result<Request<<T as IntoRequest>::Body>, Error>
  where
    T: Endpoint,
  {
    let uri = self.uri(&endpoint)?;
    let mut request = Request::builder().version(T::VERSION).method(T::METHOD).uri(uri);

    if let Some(headers) = request.headers_mut() {
      headers.typed_insert(self.authorization.clone());
      headers.typed_insert(endpoint.accept());
      headers.typed_insert(endpoint.content_type());
    }

    let body = endpoint.as_ref().clone();
    request.body(body).map_into()
  }

  pub fn callable<T>(&self, endpoint: T) -> Result<(ureq::Request, <T as IntoRequest>::Body), Error>
  where
    T: Endpoint,
  {
    let request = self.request(endpoint)?;
    let (parts, body) = request.into_parts();
    let request = ureq::Request::from(parts);
    Ok((request, body))
  }

  pub fn call<T>(&self, endpoint: T) -> Result<<T as Response>::Body, Error>
  where
    T: Endpoint,
  {
    let (request, _body) = self.callable(endpoint)?;
    let response = request.call()?;
    let body = if response.status() == 204 {
      serde_json::from_str("null")?
    } else {
      response.into_json()?
    };

    Ok(body)
  }

  pub fn paginate<T>(&self, endpoint: T) -> Result<Vec<<T as Pagination>::Item>, Error>
  where
    T: Endpoint + Pagination,
  {
    let (original_request, _body) = self.callable(endpoint)?;
    let mut request = original_request.clone();
    let mut items = Vec::new();

    let mut done = false;

    while !done {
      eprintln!("request: {}", request.url());
      let response = request.clone().call()?;

      match response
        .all(T::HEADER.as_str())
        .into_iter()
        .flat_map(parse_link_header::parse_with_rel)
        .filter_map(|mut rel| rel.remove(T::REL))
        .find_map(|mut link| link.queries.remove(T::QUERY))
      {
        Some(value) => request = original_request.clone().query(T::QUERY, &value),
        None => done = true,
      }

      let batch = response.into_json::<Vec<_>>()?;
      items.extend_from_slice(&batch);
    }

    Ok(items)
  }

  pub fn autopaginate_json<T>(&self, endpoint: T, pretty: bool) -> Result<String, Error>
  where
    T: Endpoint + Pagination,
  {
    let value = if self.auto_paginate {
      self.paginate(endpoint).map(serde_json::to_value)?
    } else {
      self.call(endpoint).map(serde_json::to_value)?
    }?;

    let json = if pretty {
      serde_json::to_string_pretty(&value)?
    } else {
      serde_json::to_string(&value)?
    };

    Ok(json)
  }

  pub fn json<T>(&self, endpoint: T, pretty: bool) -> Result<String, Error>
  where
    T: Endpoint,
  {
    let value = self.call(endpoint).map(serde_json::to_value)??;

    let json = if pretty {
      serde_json::to_string_pretty(&value)?
    } else {
      serde_json::to_string(&value)?
    };

    Ok(json)
  }
}

//   fn expected_headers() -> Vec<HeaderName> {
//     vec![
//       HeaderName::from_static("accept-ch"),
//       HeaderName::from_static("p3p"),
//       HeaderName::from_static("x-okta-request-id"),
//       HeaderName::from_static("x-rate-limit-limit"),
//       HeaderName::from_static("x-rate-limit-remaining"),
//       HeaderName::from_static("x-rate-limit-reset"),
//       http::header::ACCEPT_CHARSET,
//       http::header::CACHE_CONTROL,
//       http::header::CONNECTION,
//       http::header::CONTENT_SECURITY_POLICY,
//       http::header::CONTENT_TYPE,
//       http::header::DATE,
//       http::header::EXPIRES,
//       http::header::LINK,
//       http::header::PRAGMA,
//       http::header::REFERRER_POLICY,
//       http::header::SERVER,
//       http::header::SET_COOKIE,
//       http::header::STRICT_TRANSPORT_SECURITY,
//       http::header::TRANSFER_ENCODING,
//       http::header::VARY,
//       http::header::X_CONTENT_TYPE_OPTIONS,
//       http::header::X_XSS_PROTECTION,
//     ]
//   }
// }
