use crate::repos::announcement_repository::HaveAnnouncementRepository;
use crate::repos::class_repository::HaveClassRepository;
use crate::repos::course_repository::HaveCourseRepository;
use crate::repos::registration_course_repository::HaveRegistrationCourseRepository;
use crate::repos::registration_repository::HaveRegistrationRepository;
use crate::repos::submission_repository::HaveSubmissionRepository;
use crate::repos::unread_announcement_repository::HaveUnreadAnnouncementRepository;
use crate::repos::user_repository::HaveUserRepository;
use crate::services::HaveDBPool;

pub trait RepositoryManager:
    HaveDBPool
    + HaveAnnouncementRepository
    + HaveClassRepository
    + HaveCourseRepository
    + HaveRegistrationCourseRepository
    + HaveRegistrationRepository
    + HaveSubmissionRepository
    + HaveUnreadAnnouncementRepository
    + HaveUserRepository
{
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::db::DBPool;
    use crate::repos::announcement_repository::{
        HaveAnnouncementRepository, MockAnnouncementRepository,
    };
    use crate::repos::class_repository::{HaveClassRepository, MockClassRepository};
    use crate::repos::course_repository::{HaveCourseRepository, MockCourseRepository};
    use crate::repos::manager::RepositoryManager;
    use crate::repos::registration_course_repository::{
        HaveRegistrationCourseRepository, MockRegistrationCourseRepository,
    };
    use crate::repos::registration_repository::{
        HaveRegistrationRepository, MockRegistrationRepository,
    };
    use crate::repos::submission_repository::{HaveSubmissionRepository, MockSubmissionRepository};
    use crate::repos::unread_announcement_repository::{
        HaveUnreadAnnouncementRepository, MockUnreadAnnouncementRepository,
    };
    use crate::repos::user_repository::{HaveUserRepository, MockUserRepository};
    use crate::services::announcement_service::AnnouncementServiceImpl;
    use crate::services::course_service::CourseServiceImpl;
    use crate::services::unread_announcement_service::UnreadAnnouncementServiceImpl;
    use crate::services::HaveDBPool;

    pub struct MockRepositoryManager {
        db_pool: DBPool,
        pub announcement_repo: MockAnnouncementRepository,
        pub class_repo: MockClassRepository,
        pub course_repo: MockCourseRepository,
        pub registration_course_repo: MockRegistrationCourseRepository,
        pub registration_repo: MockRegistrationRepository,
        pub submission_repo: MockSubmissionRepository,
        pub unread_announcement_repo: MockUnreadAnnouncementRepository,
        pub user_repo: MockUserRepository,
    }

    impl MockRepositoryManager {
        pub fn new(db_pool: DBPool) -> Self {
            Self {
                db_pool,
                announcement_repo: MockAnnouncementRepository::new(),
                class_repo: MockClassRepository::new(),
                course_repo: MockCourseRepository::new(),
                registration_course_repo: MockRegistrationCourseRepository::new(),
                registration_repo: MockRegistrationRepository::new(),
                submission_repo: MockSubmissionRepository::new(),
                unread_announcement_repo: MockUnreadAnnouncementRepository::new(),
                user_repo: MockUserRepository::new(),
            }
        }
    }

    impl RepositoryManager for MockRepositoryManager {}
    impl AnnouncementServiceImpl for MockRepositoryManager {}
    impl CourseServiceImpl for MockRepositoryManager {}
    impl UnreadAnnouncementServiceImpl for MockRepositoryManager {}

    impl HaveDBPool for MockRepositoryManager {
        fn get_db_pool(&self) -> &DBPool {
            &self.db_pool
        }
    }

    impl HaveAnnouncementRepository for MockRepositoryManager {
        type Repo = MockAnnouncementRepository;

        fn announcement_repo(&self) -> &Self::Repo {
            &self.announcement_repo
        }
    }

    impl HaveClassRepository for MockRepositoryManager {
        type Repo = MockClassRepository;

        fn class_repo(&self) -> &Self::Repo {
            &self.class_repo
        }
    }

    impl HaveCourseRepository for MockRepositoryManager {
        type Repo = MockCourseRepository;

        fn course_repo(&self) -> &Self::Repo {
            &self.course_repo
        }
    }

    impl HaveRegistrationCourseRepository for MockRepositoryManager {
        type Repo = MockRegistrationCourseRepository;

        fn registration_course_repo(&self) -> &Self::Repo {
            &self.registration_course_repo
        }
    }

    impl HaveRegistrationRepository for MockRepositoryManager {
        type Repo = MockRegistrationRepository;

        fn registration_repo(&self) -> &Self::Repo {
            &self.registration_repo
        }
    }

    impl HaveSubmissionRepository for MockRepositoryManager {
        type Repo = MockSubmissionRepository;

        fn submission_repo(&self) -> &Self::Repo {
            &self.submission_repo
        }
    }

    impl HaveUnreadAnnouncementRepository for MockRepositoryManager {
        type Repo = MockUnreadAnnouncementRepository;

        fn unread_announcement_repo(&self) -> &Self::Repo {
            &self.unread_announcement_repo
        }
    }

    impl HaveUserRepository for MockRepositoryManager {
        type Repo = MockUserRepository;

        fn user_repo(&self) -> &Self::Repo {
            &self.user_repo
        }
    }
}
