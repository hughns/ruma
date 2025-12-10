//! `GET /_matrix/client/*/rendezvous/{id}`
//!
//! Get a rendezvous session.

pub mod unstable {
    //! `msc4388` ([MSC])
    //!
    //! [MSC]: https://github.com/matrix-org/matrix-spec-proposals/pull/4388

    use ruma_common::{
        api::{auth_scheme::NoAuthentication, request, response},
        metadata,
    };

    metadata! {
        method: GET,
        rate_limited: false,
        authentication: NoAuthentication,
        history: {
            unstable("io.element.msc4388") => "/_matrix/client/unstable/io.element.msc4388/rendezvous/{id}",
        }
    }

    /// Request type for the `GET` `rendezvous` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The ID of the rendezvous session to get.
        #[ruma_api(path)]
        pub id: String,
    }

    impl Request {
        /// Creates a new `Request` with the given id.
        pub fn new(id: String) -> Self {
            Self { id }
        }
    }

    #[response(error = crate::Error)]
    /// Response type for the `GET` `rendezvous` endpoint.
    pub struct Response {
        /// The current sequence token for the session.
        pub sequence_token: String,

        /// The current data for the session.
        pub data: String,
    }

    impl Response {
        /// Creates a new `Response` with the given sequence token and data.
        pub fn new(sequence_token: String, data: String) -> Self {
            Self { sequence_token, data }
        }
    }
}
