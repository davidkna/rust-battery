use std::fmt;
use std::sync::Arc;

use crate::platform::traits::{BatteryDevice, BatteryIterator, BatteryManager};
use crate::units::{ElectricPotential, Energy, Power, ThermodynamicTemperature};
use crate::{Error, Result, State, Technology};

#[derive(Default)]
pub struct FallbackManager;

impl BatteryManager for FallbackManager {
    type Iterator = FallbackIterator;

    fn new() -> Result<Self> {
        Err(Error::unsupported(
            "Support for this target OS is not implemented yet!",
        ))
    }

    fn refresh(&self, _battery: &mut FallbackDevice) -> Result<()> {
        Err(Error::unsupported(
            "Support for this target OS is not implemented yet!",
        ))
    }
}

impl fmt::Debug for FallbackManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FallbackManager").finish()
    }
}

pub struct FallbackIterator {
    #[allow(dead_code)]
    manager: Arc<FallbackManager>,
}

impl Iterator for FallbackIterator {
    type Item = Result<FallbackDevice>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl BatteryIterator for FallbackIterator {
    type Manager = FallbackManager;
    type Device = FallbackDevice;

    fn new(_manager: Arc<Self::Manager>) -> Result<Self> {
        Err(Error::unsupported(
            "Support for this target OS is not implemented yet!",
        ))
    }
}

impl fmt::Debug for FallbackIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FallbackIterator").finish()
    }
}

#[derive(Default)]
pub struct FallbackDevice;

impl BatteryDevice for FallbackDevice {
    fn energy(&self) -> Energy {
        Energy::default()
    }

    fn energy_full(&self) -> Energy {
        Energy::default()
    }

    fn energy_full_design(&self) -> Energy {
        Energy::default()
    }

    fn energy_rate(&self) -> Power {
        Power::default()
    }

    fn state(&self) -> State {
        State::Unknown
    }

    fn voltage(&self) -> ElectricPotential {
        ElectricPotential::default()
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        None
    }

    fn vendor(&self) -> Option<&str> {
        None
    }

    fn model(&self) -> Option<&str> {
        None
    }

    fn serial_number(&self) -> Option<&str> {
        None
    }

    fn technology(&self) -> Technology {
        Technology::Unknown
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }
}

impl fmt::Debug for FallbackDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FallbackDevice").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback() {
        assert!(FallbackManager::new().is_err());
        assert!(FallbackIterator::new(Arc::new(FallbackManager)).is_err());
        let mut device = FallbackDevice;
        assert!(FallbackManager.refresh(&mut device).is_err());
    }
}
