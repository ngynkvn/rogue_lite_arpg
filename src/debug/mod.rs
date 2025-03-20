use std::any::{Any, TypeId};

use bevy::ecs::schedule::SystemSchedule;
use bevy::ecs::{intern::Interned, schedule::ScheduleLabel, schedule::Schedules};
use bevy::prelude::*;
use bevy::reflect::DynamicTypePath;
use disqualified::ShortName;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct ScheduleDebugGroup;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        let schedule_name = "Update";
        let label = find_schedule(app, schedule_name).unwrap();
        app.world_mut()
            .resource_scope::<Schedules, _>(|world, mut schedules| {
                let ignored_ambiguities = schedules.ignored_scheduling_ambiguities.clone();
                let label_name = format!("{:?}", label);
                let schedule = schedules
                    .get_mut(label)
                    .ok_or_else(|| format!("schedule {label_name} doesn't exist"))
                    .unwrap();
                schedule.graph_mut().initialize(world);

                // build_schedule caches a topological sort of the dependency graph in
                // schedule_graph
                let _: SystemSchedule = schedule
                    .graph_mut()
                    .build_schedule(
                        world.components(),
                        ScheduleDebugGroup.intern(),
                        &ignored_ambiguities,
                    )
                    .unwrap();

                let graph = schedule.graph();
                let system_sets: Vec<_> = graph.system_sets().collect();
                for (set_id, system_set, condition) in system_sets {
                    info!(
                        "{:?}, {:?}",
                        set_id,
                        ShortName::from(format!("{:?}", system_set).as_str()),
                    );
                }
            });
    }
}

trait SystemSetExt {
    fn _exists(&self);
}
impl SystemSetExt for &dyn SystemSet {
    fn _exists(&self) {}
}

/// Looks up a schedule by its string name in `App`.
fn find_schedule(
    app: &App,
    schedule_name: &str,
) -> Result<Interned<dyn ScheduleLabel>, FindScheduleError> {
    let lower_schedule_name = schedule_name.to_lowercase();

    let schedules = app.world().resource::<Schedules>();
    let schedules = schedules
        .iter()
        // Note we get the Interned label from `schedule` since `&dyn ScheduleLabel` doesn't `impl
        // ScheduleLabel`.
        .map(|(label, schedule)| (format!("{label:?}").to_lowercase(), schedule.label()))
        .collect::<Vec<_>>();

    let mut found_label = None;
    for (str, label) in schedules.iter() {
        if str == &lower_schedule_name {
            if found_label.is_some() {
                return Err(FindScheduleError::MoreThanOneMatch(
                    schedule_name.to_string(),
                ));
            }
            found_label = Some(*label);
        }
    }
    found_label.ok_or(FindScheduleError::NoMatch(
        schedule_name.to_string(),
        schedules.into_iter().map(|(str, _)| str).collect(),
    ))
}
impl std::error::Error for FindScheduleError {}

enum FindScheduleError {
    /// There was no match. Holds the requested schedule, and the list of valid
    /// schedules by string.
    NoMatch(String, Vec<String>),
    MoreThanOneMatch(String),
}

impl std::fmt::Debug for FindScheduleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMatch(request, schedules) => {
                f.write_fmt(format_args!("No schedules matched the requested schedule '{request}'. The valid schedules are:\n"))?;
                for schedule in schedules {
                    f.write_fmt(format_args!("\n{schedule}"))?;
                }
                Ok(())
            }
            Self::MoreThanOneMatch(request) => f.write_fmt(format_args!(
                "More than one schedule matched requested schedule '{request}'"
            )),
        }
    }
}

impl std::fmt::Display for FindScheduleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}
