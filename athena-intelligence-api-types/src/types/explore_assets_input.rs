pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExploreAssetsInput {
    /// Only assets you hold at one of these: owner (you created it), edit (you can edit it), view (you can only view it), drive_role (set by your role in a shared drive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<ExploreAssetsInputAccessItem>>,
    /// Only these assets (ids you cannot open are left out; [] matches nothing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<String>>,
    /// Created on or after this date or time (ISO 8601; UTC). Last month: created_after=2026-10-01, created_before=2026-11-01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Created before this date or time (ISO 8601; UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only what these people created: an email, a user id (a created_by count's value), 'me', or a name, looked up among the creators of the other filters' assets. The answer's `matched` says who each name matched; a name matching more than 5 people is refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Vec<String>>,
    /// Leave these asset types out, e.g. ['session', 'folder'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_types: Option<Vec<String>>,
    /// Lists to count across the whole set, each with its top values: type, created_by, place, tag, modified_month, created_month, ownership, source, size, starred, has_summary, access (your own access: owner, edit, view only, or set by a drive role). Each list is counted with every other filter applied but not its own. None by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<ExploreAssetsInputFacetsItem>>,
    /// Only assets with (true) or without (false) a summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_summary: Option<bool>,
    /// Add each row's summary (its first 280 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_summary: Option<bool>,
    /// Rows to return (0-50). 0 returns only the total and counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Last changed on or after this date or time (ISO 8601; UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_after: Option<String>,
    /// Last changed before this date or time (ISO 8601; UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_before: Option<String>,
    /// Rows to skip, for the next page (the answer's next_offset).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// mine (you created it) or shared (someone else did).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<Vec<ExploreAssetsInputOwnershipItem>>,
    /// Only what sits directly in these folders or drives (not their subfolders): a folder or drive id, 'root' for the workspace root, or a folder name, looked up among folders you can open. The answer's `matched` says which folders each name matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub places: Option<Vec<String>>,
    /// Per-type property filters (ids in the tool's description), each {id, values, exclude, after, before}; a property may carry one pick and one exclusion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ExplorePropertyFilter>>,
    /// Per-type property ids to count, like facets (every other filter applied, not the property's own).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_facets: Option<Vec<String>>,
    /// Only assets related to this one (its folder, contents, sources, sessions, links). You must be able to open it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_to_asset_id: Option<String>,
    /// File size: none (no recorded size), lt_100kb, 100kb_1mb, 1mb_10mb, 10mb_100mb, gte_100mb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_buckets: Option<Vec<ExploreAssetsInputSizeBucketsItem>>,
    /// Order of the rows; last changed first by default. relevance puts the best title match for the words first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ExploreAssetsInputSort>,
    /// Where it came from: upload, email, sharepoint, google_drive, or athena (made in Athena).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<ExploreAssetsInputSourcesItem>>,
    /// Only assets you starred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    /// Only assets with any of these tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Values per counted list (1-10), most assets first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_n: Option<i64>,
    /// Only these asset types, e.g. ['pdf'] or ['spreadsheet', 'document']. Category words cover every type they name ('presentation', 'document').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// Words to match in titles, file names, summaries and indexed content. Every word must match; "exact phrase", OR (a OR b), -word to leave out, word* and ? wildcards, and parentheses work.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<String>,
}

impl ExploreAssetsInput {
    pub fn builder() -> ExploreAssetsInputBuilder {
        <ExploreAssetsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExploreAssetsInputBuilder {
    access: Option<Vec<ExploreAssetsInputAccessItem>>,
    asset_ids: Option<Vec<String>>,
    created_after: Option<String>,
    created_before: Option<String>,
    created_by: Option<Vec<String>>,
    exclude_types: Option<Vec<String>>,
    facets: Option<Vec<ExploreAssetsInputFacetsItem>>,
    has_summary: Option<bool>,
    include_summary: Option<bool>,
    limit: Option<i64>,
    modified_after: Option<String>,
    modified_before: Option<String>,
    offset: Option<i64>,
    ownership: Option<Vec<ExploreAssetsInputOwnershipItem>>,
    places: Option<Vec<String>>,
    properties: Option<Vec<ExplorePropertyFilter>>,
    property_facets: Option<Vec<String>>,
    related_to_asset_id: Option<String>,
    size_buckets: Option<Vec<ExploreAssetsInputSizeBucketsItem>>,
    sort: Option<ExploreAssetsInputSort>,
    sources: Option<Vec<ExploreAssetsInputSourcesItem>>,
    starred: Option<bool>,
    tags: Option<Vec<String>>,
    top_n: Option<i64>,
    types: Option<Vec<String>>,
    words: Option<String>,
}

impl ExploreAssetsInputBuilder {
    pub fn access(mut self, value: Vec<ExploreAssetsInputAccessItem>) -> Self {
        self.access = Some(value);
        self
    }

    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: Vec<String>) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn exclude_types(mut self, value: Vec<String>) -> Self {
        self.exclude_types = Some(value);
        self
    }

    pub fn facets(mut self, value: Vec<ExploreAssetsInputFacetsItem>) -> Self {
        self.facets = Some(value);
        self
    }

    pub fn has_summary(mut self, value: bool) -> Self {
        self.has_summary = Some(value);
        self
    }

    pub fn include_summary(mut self, value: bool) -> Self {
        self.include_summary = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn modified_after(mut self, value: impl Into<String>) -> Self {
        self.modified_after = Some(value.into());
        self
    }

    pub fn modified_before(mut self, value: impl Into<String>) -> Self {
        self.modified_before = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn ownership(mut self, value: Vec<ExploreAssetsInputOwnershipItem>) -> Self {
        self.ownership = Some(value);
        self
    }

    pub fn places(mut self, value: Vec<String>) -> Self {
        self.places = Some(value);
        self
    }

    pub fn properties(mut self, value: Vec<ExplorePropertyFilter>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn property_facets(mut self, value: Vec<String>) -> Self {
        self.property_facets = Some(value);
        self
    }

    pub fn related_to_asset_id(mut self, value: impl Into<String>) -> Self {
        self.related_to_asset_id = Some(value.into());
        self
    }

    pub fn size_buckets(mut self, value: Vec<ExploreAssetsInputSizeBucketsItem>) -> Self {
        self.size_buckets = Some(value);
        self
    }

    pub fn sort(mut self, value: ExploreAssetsInputSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<ExploreAssetsInputSourcesItem>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn starred(mut self, value: bool) -> Self {
        self.starred = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn top_n(mut self, value: i64) -> Self {
        self.top_n = Some(value);
        self
    }

    pub fn types(mut self, value: Vec<String>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn words(mut self, value: impl Into<String>) -> Self {
        self.words = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExploreAssetsInput`].
    pub fn build(self) -> Result<ExploreAssetsInput, BuildError> {
        Ok(ExploreAssetsInput {
            access: self.access,
            asset_ids: self.asset_ids,
            created_after: self.created_after,
            created_before: self.created_before,
            created_by: self.created_by,
            exclude_types: self.exclude_types,
            facets: self.facets,
            has_summary: self.has_summary,
            include_summary: self.include_summary,
            limit: self.limit,
            modified_after: self.modified_after,
            modified_before: self.modified_before,
            offset: self.offset,
            ownership: self.ownership,
            places: self.places,
            properties: self.properties,
            property_facets: self.property_facets,
            related_to_asset_id: self.related_to_asset_id,
            size_buckets: self.size_buckets,
            sort: self.sort,
            sources: self.sources,
            starred: self.starred,
            tags: self.tags,
            top_n: self.top_n,
            types: self.types,
            words: self.words,
        })
    }
}

