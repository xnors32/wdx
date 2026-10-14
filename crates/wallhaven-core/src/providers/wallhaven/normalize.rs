use crate::domain::{SearchRequest, SortOrder};

pub const ALLOWED_COLORS: &[&str] = &[
    "660000", "990000", "cc0000", "cc3333", "ea4c88", "993399", "663399", "333399", "0066cc",
    "0099cc", "66cccc", "77cc33", "669900", "336600", "666600", "999900", "cccc33", "ffff00",
    "ffcc33", "ff9900", "ff6600", "cc6633", "996633", "663300", "000000", "999999", "cccccc",
    "ffffff", "424153",
];

pub fn is_valid_color(value: &str) -> bool {
    ALLOWED_COLORS
        .iter()
        .any(|color| color.eq_ignore_ascii_case(value))
}

pub fn normalize_search(request: &SearchRequest, has_api_key: bool) -> Vec<(String, String)> {
    let mut params = Vec::new();

    if let Some(query) = request.query.as_ref() {
        if !query.trim().is_empty() {
            params.push(("q".to_string(), query.clone()));
        }
    }

    if request.page > 1 {
        params.push(("page".to_string(), request.page.to_string()));
    } else {
        // Keep the request explicit for page 1 as well; it matches client usage.
        params.push(("page".to_string(), request.page.max(1).to_string()));
    }

    if let Some(value) = request.filters.categories.as_ref() {
        if !value.trim().is_empty() {
            params.push(("categories".to_string(), value.clone()));
        }
    }

    let mut purity = request.filters.purity.clone();
    if let Some(value) = purity.as_mut() {
        let chars: Vec<char> = value.chars().collect();
        if chars.len() == 3 && chars[2] == '1' && !has_api_key {
            let mut normalized: Vec<char> = chars;
            normalized[2] = '0';
            *value = normalized.into_iter().collect();
        }
    }
    if let Some(value) = purity {
        if !value.trim().is_empty() {
            params.push(("purity".to_string(), value));
        }
    }

    if let Some(value) = request.filters.sorting.as_ref() {
        if !value.trim().is_empty() {
            params.push(("sorting".to_string(), value.clone()));
        }
    }

    if let Some(value) = request.filters.order {
        params.push((
            "order".to_string(),
            match value {
                SortOrder::Desc => "desc".to_string(),
                SortOrder::Asc => "asc".to_string(),
            },
        ));
    }

    if request.filters.sorting.as_deref() == Some("toplist") {
        if let Some(value) = request.filters.top_range.as_ref() {
            if !value.trim().is_empty() {
                params.push(("topRange".to_string(), value.clone()));
            }
        }
    }

    if let Some(value) = request.filters.atleast.as_ref() {
        if !value.trim().is_empty() {
            params.push(("atleast".to_string(), value.clone()));
        }
    }

    if let Some(value) = request.filters.resolutions.as_ref() {
        if !value.trim().is_empty() {
            params.push(("resolutions".to_string(), value.clone()));
        }
    }

    if let Some(value) = request.filters.ratios.as_ref() {
        if !value.trim().is_empty() {
            params.push(("ratios".to_string(), value.clone()));
        }
    }

    if let Some(value) = request.filters.colors.as_ref() {
        if !value.trim().is_empty() {
            // Color validation is enforced by the caller/client; the parameter name uses
            // the spelling required by Wallhaven.
            params.push(("colors".to_string(), value.clone()));
        }
    }

    if request.filters.sorting.as_deref() == Some("random") {
        if let Some(seed) = request.seed.as_ref() {
            if !seed.trim().is_empty() {
                params.push(("seed".to_string(), seed.clone()));
            }
        }
    }

    params
}
