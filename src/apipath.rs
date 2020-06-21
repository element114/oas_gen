use contracts::pre;

use crate::okapi3::Parameter;
use crate::queryparam::QueryParamBuilder;

#[derive(Debug, Clone, Default)]
pub struct ApiId {
    pub document: String,
    pub key: String,
    // Use new or default please
    nothing: (),
}
impl std::fmt::Display for ApiId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.document, self.key)
    }
}
impl ApiId {
    #[pre(!document.contains('/'))]
    #[pre(key.starts_with('{'))]
    #[pre(key.ends_with('}'))]
    pub fn new(document: &str, key: &str) -> Self {
        ApiId {
            document: document.to_owned(),
            key: key.to_owned(),
            nothing: (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiPath {
    pub prefix: Option<String>,
    pub ids: Vec<ApiId>,
    pub token: Option<String>,
    pub(crate) query_params: Vec<Parameter>,
}
impl std::fmt::Display for ApiPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp = vec![];
        let ids: Vec<String> = self.ids.iter().map(ToString::to_string).collect();
        if let Some(pfx) = &self.prefix {
            tmp.push(pfx.clone());
        }
        tmp.extend(ids);
        if let Some(tkn) = &self.token {
            tmp.push(tkn.clone());
        }
        let pth: String = tmp.join("/");

        write!(f, "/{}", pth)
    }
}
impl ApiPath {
    /// </api/testdoc> is represented as @prefix:`api` and @token:`testdoc`.
    /// </api/user/8/testdoc> is represented as @prefix:`api` @ids: `[('user','{user_key}')]` and @token:`testdoc`.
    #[pre(!prefix.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('{'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('}'))]
    #[pre(!token.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!token.clone().unwrap_or_default().contains('{'))]
    #[pre(!token.clone().unwrap_or_default().contains('}'))]
    pub fn new(prefix: Option<String>, ids: Vec<ApiId>, token: Option<String>) -> Self {
        ApiPath {
            prefix,
            ids,
            token,
            query_params: vec![],
        }
    }

    /// Adds query parameters to the url, otherwise same as new.
    #[pre(!prefix.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('{'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('}'))]
    #[pre(!token.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!token.clone().unwrap_or_default().contains('{'))]
    #[pre(!token.clone().unwrap_or_default().contains('}'))]
    pub fn with_queries(
        prefix: Option<String>,
        ids: Vec<ApiId>,
        token: Option<String>,
        qpbuilders: Vec<QueryParamBuilder>,
    ) -> Self {
        let query_params = qpbuilders.iter().map(QueryParamBuilder::build).collect();
        ApiPath {
            prefix,
            ids,
            token,
            query_params,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ApiId;
    use super::ApiPath;

    #[test]
    fn test_api_path() {
        let test_path = ApiPath::new(Some("api".to_owned()), vec![], Some("testdoc".to_owned()));
        let test_str = test_path.to_string();
        assert_eq!("/api/testdoc", test_str.as_str());

        let test_path = ApiPath::new(Some("api/testdoc".to_owned()), vec![], None);
        let test_str = test_path.to_string();
        assert_eq!("/api/testdoc", test_str.as_str());

        let test_path = ApiPath::new(
            Some("api".to_owned()),
            vec![ApiId::new("parents", "{pid}")],
            Some("testdoc".to_owned()),
        );
        let test_str = test_path.to_string();
        assert_eq!("/api/parents/{pid}/testdoc", test_str.as_str());
    }
}
