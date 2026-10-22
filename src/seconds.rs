use std::time::Duration;

pub(crate) fn seconds(duration: Duration) -> u32 {
    u32::try_from(duration.as_secs()).unwrap_or(u32::MAX)
}

pub(crate) fn millis(duration: Duration) -> u32 {
    u32::try_from(duration.as_millis()).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_become_whole_units() {
        assert_eq!(seconds(Duration::from_millis(1_500)), 1);
        assert_eq!(seconds(Duration::from_secs(3_600)), 3_600);
        assert_eq!(seconds(Duration::MAX), u32::MAX);
        assert_eq!(millis(Duration::from_secs(5)), 5_000);
        assert_eq!(millis(Duration::MAX), u32::MAX);
    }
}
