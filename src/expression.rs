// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

//! The main struct of the library.

use std::fmt::Display;

use crate::{error::SpdxExpressionError, expression_variant::ExpressionVariant};

/// Main struct for SPDX License Expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpdxExpression {
    /// The parsed expression.
    inner: ExpressionVariant,
}

impl SpdxExpression {
    /// Parse `Self` from a string. The input expression needs to be a syntactically valid SPDX
    /// expression, `NONE` or `NOASSERTION`. The parser accepts license identifiers that are not
    /// valid SPDX.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spdx_expression::SpdxExpression;
    /// # use spdx_expression::SpdxExpressionError;
    /// #
    /// let expression = SpdxExpression::parse("MIT")?;
    /// # Ok::<(), SpdxExpressionError>(())
    /// ```
    ///
    /// License expressions need to be syntactically valid, but they can include license
    /// identifiers not on the SPDX license list or not specified with `LicenseRef`.
    ///
    /// ```
    /// # use spdx_expression::SpdxExpression;
    /// # use spdx_expression::SpdxExpressionError;
    /// #
    /// let expression = SpdxExpression::parse("MIT OR InvalidLicenseId")?;
    /// # Ok::<(), SpdxExpressionError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `SpdxExpressionError` if the license expression is not syntactically valid.
    pub fn parse(expression: &str) -> Result<Self, SpdxExpressionError> {
        Ok(Self {
            inner: ExpressionVariant::parse(expression)
                .map_err(|err| SpdxExpressionError::Parse(err.to_string()))?,
        })
    }

    /// Get all license and exception identifiers from the `SpdxExpression`. Returns the licenses
    /// alphabetically sorted and deduped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spdx_expression::SpdxExpression;
    /// # use spdx_expression::SpdxExpressionError;
    /// #
    /// let expression = SpdxExpression::parse("MIT OR Apache-2.0")?;
    /// let licenses = expression.licenses();
    /// assert_eq!(licenses, vec!["Apache-2.0".to_string(), "MIT".to_string()]);
    /// # Ok::<(), SpdxExpressionError>(())
    /// ```
    pub fn licenses(&self) -> Vec<String> {
        let expression_string = self.to_string();
        let licenses = expression_string.split_ascii_whitespace();
        let licenses = licenses.filter(|&i| i != "OR" && i != "AND" && i != "WITH");
        let licenses = licenses.map(|i| i.replace('(', "").replace(')', ""));
        let mut licenses = licenses.collect::<Vec<_>>();
        licenses.sort_unstable();
        licenses.dedup();
        licenses
    }
}

impl Display for SpdxExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_works() {
        let expression = SpdxExpression::parse("MIT AND (Apache-2.0 OR ISC)").unwrap();
        assert_eq!(expression.to_string(), "MIT AND (Apache-2.0 OR ISC)");
    }

    #[test]
    fn test_licenses_from_simple_expression() {
        let expression = SpdxExpression::parse("MIT").unwrap();
        let licenses = expression.licenses();
        assert_eq!(licenses, vec!["MIT".to_string()]);
    }

    #[test]
    fn test_licenses_from_compound_or_expression() {
        let expression = SpdxExpression::parse("MIT OR Apache-2.0").unwrap();
        let licenses = expression.licenses();
        assert_eq!(licenses, vec!["Apache-2.0".to_string(), "MIT".to_string()]);
    }

    #[test]
    fn test_licenses_from_compound_parentheses_expression() {
        let expression = SpdxExpression::parse(
            "(MIT OR Apache-2.0 AND (GPL-2.0-only WITH Classpath-exception-2.0 OR ISC))",
        )
        .unwrap();
        let licenses = expression.licenses();
        assert_eq!(
            licenses,
            vec![
                "Apache-2.0".to_string(),
                "Classpath-exception-2.0".to_string(),
                "GPL-2.0-only".to_string(),
                "ISC".to_string(),
                "MIT".to_string()
            ]
        );
    }
}
