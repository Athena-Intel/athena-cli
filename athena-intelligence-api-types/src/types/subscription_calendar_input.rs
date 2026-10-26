pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A business calendar a cron is read against.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionCalendarInput {
    /// Skip Saturday and Sunday fires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_days: Option<bool>,
    /// A named set (us_federal, uk_bank, ca_federal, au_national) or a list of ISO dates such as ["2026-12-25"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holidays: Option<SubscriptionCalendarInputHolidays>,
    /// What a fire on a non-business day does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missed: Option<SubscriptionCalendarInputMissed>,
}

impl SubscriptionCalendarInput {
    pub fn builder() -> SubscriptionCalendarInputBuilder {
        <SubscriptionCalendarInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionCalendarInputBuilder {
    business_days: Option<bool>,
    holidays: Option<SubscriptionCalendarInputHolidays>,
    missed: Option<SubscriptionCalendarInputMissed>,
}

impl SubscriptionCalendarInputBuilder {
    pub fn business_days(mut self, value: bool) -> Self {
        self.business_days = Some(value);
        self
    }

    pub fn holidays(mut self, value: SubscriptionCalendarInputHolidays) -> Self {
        self.holidays = Some(value);
        self
    }

    pub fn missed(mut self, value: SubscriptionCalendarInputMissed) -> Self {
        self.missed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionCalendarInput`].
    pub fn build(self) -> Result<SubscriptionCalendarInput, BuildError> {
        Ok(SubscriptionCalendarInput {
            business_days: self.business_days,
            holidays: self.holidays,
            missed: self.missed,
        })
    }
}
