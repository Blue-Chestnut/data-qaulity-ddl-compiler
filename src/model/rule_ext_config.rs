#[derive(Clone, Debug, PartialEq)]
pub struct RuleExtConfig {
    name: String,
    description: String,
    priority: u32,
    enabled: bool,
    schedule_enabled: bool,
    schedule_frequency: ScheduleFrequency,
    // future ideas:
    // - schedule_time: String
    // - schedule_date: String
    // - alarm_threshold: String
    // - alarm_notification: AlarmNotification
    // - custom_title: String
    // - labels: Vec<Label>
}

impl Default for RuleExtConfig{
    fn default() -> Self {
        RuleExtConfig::new_empty()
    }
}

impl RuleExtConfig {
    pub fn new_empty() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            priority: 0,
            enabled: false,
            schedule_enabled: false,
            schedule_frequency: ScheduleFrequency::Daily,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScheduleFrequency {
    Daily,
    // Weekly,
    // Monthly,
    // Yearly,
    // Custom { frequency: String },
}
