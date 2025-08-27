//! HTTP client macros for reducing API method boilerplate

/// Macro to generate simple HTTP GET method wrappers
#[macro_export]
macro_rules! http_get {
    // Simple GET request: method_name -> path
    ($method_name:ident, $path:literal, $return_type:ty) => {
        #[doc = concat!("Makes a GET request to ", $path)]
        pub async fn $method_name(&self) -> $crate::error::Result<$return_type> {
            self.http_client.get($path).await
        }
    };

    // GET with AsRef<str> parameter (must come first to match before generic pattern)
    ($method_name:ident, $path_fmt:literal, $param:ident: impl AsRef<str>, $return_type:ty) => {
        #[doc = concat!("Makes a GET request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: impl AsRef<str>,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param.as_ref());
            self.http_client.get(&path).await
        }
    };

    // GET with path parameter: method_name(&self, id) -> format!("/path/{}", id)
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a GET request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.get(&path).await
        }
    };
}

/// Macro to generate HTTP GET method wrappers with beta headers
#[macro_export]
macro_rules! http_get_beta {
    // GET with beta headers and Into<String> parameter (must come before generic pattern)
    ($method_name:ident, $path_fmt:literal, $param:ident: impl Into<String>, $return_type:ty) => {
        #[doc = concat!("Makes a GET request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: impl Into<String>,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param.into());
            self.http_client.get_with_beta(&path).await
        }
    };

    // GET with beta headers and path parameter
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a GET request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.get_with_beta(&path).await
        }
    };
}

/// Macro to generate HTTP POST method wrappers
#[macro_export]
macro_rules! http_post {
    // POST with body: method_name(&self, request) -> path
    ($method_name:ident, $path:literal, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request to ", $path)]
        pub async fn $method_name(
            &self,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            self.http_client.post($path, $request).await
        }
    };

    // POST with path parameter and body
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.post(&path, $request).await
        }
    };
}

/// Macro to generate HTTP POST method wrappers with beta headers
#[macro_export]
macro_rules! http_post_beta {
    // POST with beta headers and body
    ($method_name:ident, $path:literal, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request with beta headers to ", $path)]
        pub async fn $method_name(
            &self,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            self.http_client.post_with_beta($path, $request).await
        }
    };

    // POST with beta headers, path parameter and body
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.post_with_beta(&path, $request).await
        }
    };
}

/// Macro to generate HTTP DELETE method wrappers
#[macro_export]
macro_rules! http_delete {
    // DELETE with path parameter
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a DELETE request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.delete(&path).await
        }
    };
}

/// Macro to generate HTTP DELETE method wrappers with beta headers
#[macro_export]
macro_rules! http_delete_beta {
    // DELETE with beta headers and Into<String> parameter (must come before generic pattern)
    ($method_name:ident, $path_fmt:literal, $param:ident: impl Into<String>, $return_type:ty) => {
        #[doc = concat!("Makes a DELETE request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: impl Into<String>,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param.into());
            self.http_client.delete_with_beta(&path).await
        }
    };

    // DELETE with beta headers and path parameter
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a DELETE request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.delete_with_beta(&path).await
        }
    };
}
