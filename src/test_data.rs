use crate::model::*;

pub fn make_course() -> Course {
    Course {
        id: 54321,
        url: "the-url".into(),
        published_title: "css-the-complete-guide-incl-flexbox-grid-sass".into(),
    }
}

pub fn make_test_course_content() -> CourseContent {
    CourseContent {
        chapters: vec![Chapter {
            object_index: 1,
            title: "The Chapter".into(),
            lectures: vec![Lecture {
                has_video: true,
                filename: "blah-blah.mp4".into(),
                id: 4321,
                object_index: 1,
                title: "The Lecture".into(),
            }],
        }],
    }
}
