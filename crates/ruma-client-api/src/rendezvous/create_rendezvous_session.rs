//! `POST /_matrix/client/*/rendezvous/`
//!
//! Create a rendezvous session.

pub mod unstable {
    //! `msc4108` ([MSC])
    //!
    //! [MSC]: https://github.com/matrix-org/matrix-spec-proposals/pull/4108
    use ruma_common::{
        api::{auth_scheme::AccessTokenOptional, request, response},
        metadata,
    };

    metadata! {
        method: POST,
        rate_limited: true,
        authentication: AccessTokenOptional,
        history: {
            unstable("io.element.msc4108") => "/_matrix/client/unstable/io.element.msc4108/rendezvous",
        }
    }

    /// Request type for the `POST` `rendezvous` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// Data up to maximum size allowed by the server.
        pub data: String,
    }

    impl Request {
        /// Creates a new `Request` with the given content.
        pub fn new(data: String) -> Self {
            Self { data }
        }
    }

    /// Response type for the `POST` `rendezvous` endpoint.
    #[response(error = crate::Error)]
    pub struct Response {
        /// The ID of the created rendezvous session.
        pub id: String,
        /// The initial sequence token for the session.
        pub sequence_token: String,
    }

    impl Response {
        /// Creates a new `Response` with the given content.
        pub fn new(id: String, sequence_token: String) -> Self {
            Self { id, sequence_token }
        }
    }
}
