use bevy::prelude::{Added, BuildChildren, Commands, Entity, Query, With, Res, Transform, Without, Component, info};

use crate::{resources::{XrFrameState, XrInstance, XrSession}, input::XrInput};

use super::{oculus_touch::OculusController, Hand, Vec3Conv, QuatConv};

#[derive(Component)]
pub struct OpenXRTrackingRoot;
#[derive(Component)]
pub struct OpenXRTracker;
#[derive(Component)]
pub struct OpenXRLeftEye;
#[derive(Component)]
pub struct OpenXRRightEye;
#[derive(Component)]
pub struct OpenXRHMD;
#[derive(Component)]
pub struct OpenXRLeftController;
#[derive(Component)]
pub struct OpenXRRightController;
#[derive(Component)]
pub struct OpenXRController;

pub fn adopt_open_xr_trackers(
    query: Query<Entity, Added<OpenXRTracker>>,
    mut commands: Commands,
    tracking_root_query: Query<(Entity, With<OpenXRTrackingRoot>)>,
) {
    let root = tracking_root_query.get_single();
    match root {
        Ok(thing) => {
            // info!("root is");
            for tracker in query.iter() {
                info!("we got a new tracker");
                commands.entity(thing.0).add_child(tracker);
            }
        }
        Err(_) => info!("root isnt spawned yet?"),
    }
}

pub fn update_open_xr_controllers(
    oculus_controller: Res<OculusController>,
    mut left_controller_query: Query<(
        &mut Transform,
        With<OpenXRLeftController>,
        Without<OpenXRRightController>,
    )>,
    mut right_controller_query: Query<(
        &mut Transform,
        With<OpenXRRightController>,
        Without<OpenXRLeftController>,
    )>,
    frame_state: Res<XrFrameState>,
    instance: Res<XrInstance>,
    xr_input: Res<XrInput>,
    session: Res<XrSession>,
) {
    //lock dat frame?
    let frame_state = *frame_state.lock().unwrap();
    //get controller
    let controller = oculus_controller.get_ref(&instance, &session, &frame_state, &xr_input);
    //get left controller
    let left = controller.grip_space(Hand::Left);
    let left_postion = left.0.pose.position.to_vec3();

    left_controller_query
        .get_single_mut()
        .unwrap()
        .0
        .translation = left_postion;

    left_controller_query.get_single_mut().unwrap().0.rotation = left.0.pose.orientation.to_quat();
    //get right controller
    let right = controller.grip_space(Hand::Right);
    let right_postion = right.0.pose.position.to_vec3();

    right_controller_query
        .get_single_mut()
        .unwrap()
        .0
        .translation = right_postion;

    right_controller_query.get_single_mut().unwrap().0.rotation =
        right.0.pose.orientation.to_quat();
}

