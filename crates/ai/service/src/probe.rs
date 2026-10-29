//! Probing the machines for the model page, apart from calls: no lock is
//! taken, so it runs while a call does, and settings may change meanwhile.
//! See dev_docs/Pipeline.md「用途和模型」.

use std::sync::PoisonError;
use std::time::Duration;

use ai_invoker::{Machine, MachineReport};
use futures_util::future::join_all;
use serde::Serialize;

use crate::{AiError, AiService};

/// One machine of the settings, as the last probe found it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct MachineProbe {
    pub machine: String,
    pub report: MachineReport,
}

impl AiService {
    /// Probes one machine, which may be one not saved yet; one that saving
    /// would refuse is refused. Nothing is kept.
    pub async fn probe(&self, machine: &Machine) -> Result<MachineReport, AiError> {
        Ok(ai_invoker::probe(machine, self.connect_timeout()).await?)
    }

    /// Probes every machine in the settings at once and keeps the reports,
    /// in the settings' order, in place of the last ones.
    pub async fn probe_all(&self) -> Vec<MachineProbe> {
        let settings = self.settings();
        let timeout = self.connect_timeout();
        let probes = settings.invoker.machines.iter().map(|machine| async move {
            MachineProbe {
                machine: machine.name.clone(),
                // Saved machines have passed the checks already.
                report: ai_invoker::probe(machine, timeout)
                    .await
                    .unwrap_or_else(|error| MachineReport::Offline {
                        reason: error.to_string(),
                    }),
            }
        });
        let reports = join_all(probes).await;
        *self.reports.lock().unwrap_or_else(PoisonError::into_inner) = reports.clone();
        reports
    }

    /// What the last `probe_all` found; empty before the first.
    pub fn reports(&self) -> Vec<MachineProbe> {
        self.reports
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.settings().invoker.connect_timeout_secs)
    }
}
