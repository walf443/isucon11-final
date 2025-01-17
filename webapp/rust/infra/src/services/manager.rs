use crate::services::announcement_service::AnnouncementServiceInfra;
use crate::services::class_service::ClassServiceInfra;
use crate::services::grade_summary_service::GradeSummaryServiceInfra;
use crate::services::registration_course_service::RegistrationCourseServiceInfra;
use crate::services::submission_service::SubmissionServiceInfra;
use crate::services::unread_announcement_service::UnreadAnnouncementServiceInfra;
use crate::services::user_service::UserServiceInfra;
use crate::services::CourseServiceInfra;
use isucholar_core::db::DBPool;
use isucholar_core::services::announcement_service::HaveAnnouncementService;
use isucholar_core::services::class_service::HaveClassService;
use isucholar_core::services::course_service::HaveCourseService;
use isucholar_core::services::grade_summary_service::HaveGradeSummaryService;
use isucholar_core::services::manager::ServiceManager;
use isucholar_core::services::registration_course_service::HaveRegistrationCourseService;
use isucholar_core::services::submission_service::HaveSubmissionService;
use isucholar_core::services::unread_announcement_service::HaveUnreadAnnouncementService;
use isucholar_core::services::user_service::HaveUserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceManagerInfra {
    announcement_service: AnnouncementServiceInfra,
    unread_announcement_service: UnreadAnnouncementServiceInfra,
    user_service: UserServiceInfra,
    course_service: CourseServiceInfra,
    class_service: ClassServiceInfra,
    registration_course_service: RegistrationCourseServiceInfra,
    grade_summary_service: GradeSummaryServiceInfra,
    submission_service: SubmissionServiceInfra,
}

impl ServiceManager for ServiceManagerInfra {}

impl ServiceManagerInfra {
    pub fn new(db_pool: DBPool) -> Self {
        let pool = Arc::new(db_pool);
        Self {
            announcement_service: AnnouncementServiceInfra::new(pool.clone()),
            unread_announcement_service: UnreadAnnouncementServiceInfra::new(pool.clone()),
            user_service: UserServiceInfra::new(pool.clone()),
            course_service: CourseServiceInfra::new(pool.clone()),
            class_service: ClassServiceInfra::new(pool.clone()),
            registration_course_service: RegistrationCourseServiceInfra::new(pool.clone()),
            grade_summary_service: GradeSummaryServiceInfra::new(pool.clone()),
            submission_service: SubmissionServiceInfra::new(pool),
        }
    }
}

impl HaveAnnouncementService for ServiceManagerInfra {
    type Service = AnnouncementServiceInfra;

    fn announcement_service(&self) -> &Self::Service {
        &self.announcement_service
    }
}

impl HaveCourseService for ServiceManagerInfra {
    type Service = CourseServiceInfra;

    fn course_service(&self) -> &Self::Service {
        &self.course_service
    }
}

impl HaveUnreadAnnouncementService for ServiceManagerInfra {
    type Service = UnreadAnnouncementServiceInfra;

    fn unread_announcement_service(&self) -> &Self::Service {
        &self.unread_announcement_service
    }
}
impl HaveUserService for ServiceManagerInfra {
    type Service = UserServiceInfra;

    fn user_service(&self) -> &Self::Service {
        &self.user_service
    }
}
impl HaveClassService for ServiceManagerInfra {
    type Service = ClassServiceInfra;

    fn class_service(&self) -> &Self::Service {
        &self.class_service
    }
}

impl HaveRegistrationCourseService for ServiceManagerInfra {
    type Service = RegistrationCourseServiceInfra;

    fn registration_course_service(&self) -> &Self::Service {
        &self.registration_course_service
    }
}

impl HaveGradeSummaryService for ServiceManagerInfra {
    type Service = GradeSummaryServiceInfra;

    fn grade_summary_service(&self) -> &Self::Service {
        &self.grade_summary_service
    }
}

impl HaveSubmissionService for ServiceManagerInfra {
    type Service = SubmissionServiceInfra;

    fn submission_service(&self) -> &Self::Service {
        &self.submission_service
    }
}
