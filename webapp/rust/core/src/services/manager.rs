use crate::services::announcement_service::HaveAnnouncementService;
use crate::services::class_service::HaveClassService;
use crate::services::course_service::HaveCourseService;
use crate::services::grade_summary_service::HaveGradeSummaryService;
use crate::services::registration_course_service::HaveRegistrationCourseService;
use crate::services::unread_announcement_service::HaveUnreadAnnouncementService;
use crate::services::user_service::HaveUserService;

pub trait ServiceManager:
    HaveUnreadAnnouncementService
    + HaveAnnouncementService
    + HaveUserService
    + HaveCourseService
    + HaveClassService
    + HaveRegistrationCourseService
    + HaveGradeSummaryService
{
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use crate::services::announcement_service::{HaveAnnouncementService, MockAnnouncementService};
    use crate::services::class_service::{HaveClassService, MockClassService};
    use crate::services::course_service::{HaveCourseService, MockCourseService};
    use crate::services::grade_summary_service::{
        HaveGradeSummaryService, MockGradeSummaryService,
    };
    use crate::services::manager::ServiceManager;
    use crate::services::registration_course_service::{
        HaveRegistrationCourseService, MockRegistrationCourseService,
    };
    use crate::services::unread_announcement_service::{
        HaveUnreadAnnouncementService, MockUnreadAnnouncementService,
    };
    use crate::services::user_service::{HaveUserService, MockUserService};

    pub struct MockServiceManager {
        pub unread_announcement_service: MockUnreadAnnouncementService,
        pub announcement_service: MockAnnouncementService,
        pub user_service: MockUserService,
        pub course_service: MockCourseService,
        pub class_service: MockClassService,
        pub registration_course_service: MockRegistrationCourseService,
        pub grade_summary_service: MockGradeSummaryService,
    }

    impl MockServiceManager {
        pub fn new() -> Self {
            Self {
                unread_announcement_service: MockUnreadAnnouncementService::new(),
                announcement_service: MockAnnouncementService::new(),
                user_service: MockUserService::new(),
                course_service: MockCourseService::new(),
                class_service: MockClassService::new(),
                registration_course_service: MockRegistrationCourseService::new(),
                grade_summary_service: MockGradeSummaryService::new(),
            }
        }
    }

    impl ServiceManager for MockServiceManager {}

    impl HaveUnreadAnnouncementService for MockServiceManager {
        type Service = MockUnreadAnnouncementService;

        fn unread_announcement_service(&self) -> &Self::Service {
            &self.unread_announcement_service
        }
    }

    impl HaveAnnouncementService for MockServiceManager {
        type Service = MockAnnouncementService;

        fn announcement_service(&self) -> &Self::Service {
            &self.announcement_service
        }
    }

    impl HaveUserService for MockServiceManager {
        type Service = MockUserService;

        fn user_service(&self) -> &Self::Service {
            &self.user_service
        }
    }

    impl HaveCourseService for MockServiceManager {
        type Service = MockCourseService;

        fn course_service(&self) -> &Self::Service {
            &self.course_service
        }
    }

    impl HaveClassService for MockServiceManager {
        type Service = MockClassService;

        fn class_service(&self) -> &Self::Service {
            &self.class_service
        }
    }

    impl HaveRegistrationCourseService for MockServiceManager {
        type Service = MockRegistrationCourseService;

        fn registration_course_service(&self) -> &Self::Service {
            &self.registration_course_service
        }
    }

    impl HaveGradeSummaryService for MockServiceManager {
        type Service = MockGradeSummaryService;

        fn grade_summary_service(&self) -> &Self::Service {
            &self.grade_summary_service
        }
    }
}
