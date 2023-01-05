use std::fmt::Debug;

pub trait Because {
    type Output;

    fn because(
        self,
        code: Option<super::Code>,
        reasons: Option<impl Into<Vec<String>>>,
    ) -> Result<Self::Output, super::Response>;
}

impl<T, E> Because for Result<T, E>
where
    E: Into<super::Raw> + Debug,
    T: Debug,
{
    type Output = T;

    fn because(
        self,
        code: Option<super::Code>,
        reasons: Option<impl Into<Vec<String>>>,
    ) -> Result<Self::Output, super::Response> {
        match self {
            Ok(a) => Ok(a),
            Err(e) => Err(super::Response::from_raw(
                self.unwrap_err().into(),
                code,
                reasons
                    .map(|x| x.into())
                    .unwrap_or_else(|| Vec::with_capacity(0)),
            )),
        }
    }
}
