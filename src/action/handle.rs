//! Goal handle representing each action goal.

use parking_lot::Mutex;
use std::{collections::BTreeMap, rc::Rc, sync::Arc};

use super::{server::ServerData, GoalEvent, GoalStatus};
use crate::{
    error::{DynError, RCLActionError, RCLActionResult},
    logger::{pr_error_in, Logger},
    msg::ActionMsg,
    rcl,
};

/// GoalHandle contains information about an action goal and is used by server worker threads to send feedback and results.
pub struct GoalHandle<T: ActionMsg> {
    pub goal_id: [u8; 16],
    pub(crate) handle: Rc<GoalHandleData>,
    data: Arc<ServerData>,
    pub results: Arc<Mutex<BTreeMap<[u8; 16], T::ResultContent>>>,
}

impl<T> Clone for GoalHandle<T>
where
    T: ActionMsg,
{
    fn clone(&self) -> Self {
        Self {
            goal_id: self.goal_id,
            handle: self.handle.clone(),
            data: self.data.clone(),
            results: self.results.clone(),
        }
    }
}

impl<T> GoalHandle<T>
where
    T: ActionMsg,
{
    pub(crate) fn new(
        goal_id: [u8; 16],
        goal_handle: *mut rcl::rcl_action_goal_handle_t,
        data: Arc<ServerData>,
        results: Arc<Mutex<BTreeMap<[u8; 16], T::ResultContent>>>,
    ) -> Self {
        Self {
            goal_id,
            handle: Rc::new(GoalHandleData(goal_handle)),
            data,
            results,
        }
    }

    /// Publish a feedback.
    pub fn feedback(&self, content: T::FeedbackContent) -> Result<(), DynError> {
        let mut msg = <T as ActionMsg>::new_feedback_message(content, self.goal_id);

        let guard = rcl::MT_UNSAFE_FN.lock();
        guard.rcl_action_publish_feedback(
            unsafe { self.data.as_ptr_mut() },
            &mut msg as *const _ as *mut _,
        )?;
        Ok(())
    }

    /// Notify the server that the goal is successfully canceled.
    pub fn canceled(&self, result: T::ResultContent) -> Result<(), DynError> {
        self.update_result(result)?;

        self.update(GoalEvent::Canceled)?;
        self.data.publish_goal_status()?;

        Ok(())
    }

    /// Notify the server that the goal is successfully finished.
    pub fn finish(&self, result: T::ResultContent) -> Result<(), DynError> {
        self.update_result(result)?;

        self.update(GoalEvent::Succeed)?;
        self.data.publish_goal_status()?;

        Ok(())
    }

    pub fn is_canceling(&self) -> Result<bool, DynError> {
        Ok(GoalStatus::Canceling == self.status()?)
    }

    /// Returns true if the goal is in a terminal state (succeeded, canceled, or aborted).
    pub fn is_terminal(&self) -> Result<bool, DynError> {
        let s = self.status()?;
        Ok(GoalStatus::Succeeded == s || GoalStatus::Canceled == s || GoalStatus::Aborted == s)
    }

    pub fn abort(&self) -> Result<(), RCLActionError> {
        self.update(GoalEvent::Abort)?;
        self.data.publish_goal_status()?;
        Ok(())
    }

    pub(crate) fn update(&self, event: GoalEvent) -> Result<(), RCLActionError> {
        self.handle.update_goal_state(event)
    }

    pub(crate) fn send_available_results(
        &self,
        goal_id: [u8; 16],
        result: T::ResultContent,
    ) -> RCLActionResult<()> {
        let mut requests = self.data.pending_result_requests.lock();
        let guard = rcl::MT_UNSAFE_FN.lock();

        if let Some(reqs) = requests.remove(&goal_id) {
            for mut request_id in reqs {
                let response = T::new_result_response(GoalStatus::Succeeded as u8, result.clone());

                match guard.rcl_action_send_result_response(
                    unsafe { self.data.as_ptr_mut() },
                    &mut request_id,
                    &response as *const _ as *mut _,
                ) {
                    Ok(()) => {}
                    Err(e) => {
                        let logger = Logger::new("safe_drive");
                        pr_error_in!(
                            logger,
                            "failed to send result response from action server: {}",
                            e
                        );
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }

    fn update_result(&self, result: T::ResultContent) -> Result<(), DynError> {
        let mut results = self.results.lock();
        if results.insert(self.goal_id, result.clone()).is_some() {
            return Err(format!(
                "the result for the goal (id: {:?}) already exists; it should be set only once",
                self.goal_id
            )
            .into());
        }

        self.send_available_results(self.goal_id, result)?;

        Ok(())
    }

    fn status(&self) -> Result<GoalStatus, DynError> {
        let mut s: rcl::rcl_action_goal_state_t = GoalStatus::Unknown as i8;
        let guard = rcl::MT_UNSAFE_FN.lock();
        guard
            .rcl_action_goal_handle_get_status(self.handle.0, &mut s)
            .unwrap();

        Ok(GoalStatus::from(s))
    }
}

unsafe impl<T> Send for GoalHandle<T> where T: ActionMsg {}
unsafe impl<T> Sync for GoalHandle<T> where T: ActionMsg {}

/// `GoalHandleData` wraps the pointer to `rcl_action_goal_handle_t` and finalizes it when dropped.
pub(crate) struct GoalHandleData(pub *mut rcl::rcl_action_goal_handle_t);

impl GoalHandleData {
    pub(crate) fn update_goal_state(&self, event: GoalEvent) -> Result<(), RCLActionError> {
        let guard = rcl::MT_UNSAFE_FN.lock();
        guard.rcl_action_update_goal_state(self.0, event as u32)?;

        Ok(())
    }
}

impl Drop for GoalHandleData {
    fn drop(&mut self) {
        let guard = rcl::MT_UNSAFE_FN.lock();
        let _ = guard.rcl_action_goal_handle_fini(self.0);
    }
}
