use crate::models::*;
use axum::{http::StatusCode, response::IntoResponse, Json};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    // TODO: Create a POST route to /question which accepts a `Question` and returns `QuestionDetail` as JSON.
    (StatusCode::OK, 
        Json(
            QuestionDetail {
                question_uuid: String::from("d347261c-3f0e-42d2-8706-5ef9f1b96725"),
                title: question.title.clone(),
                description: question.description.clone(),
                created_at: String::from("2024-09-11T12:00:00Z"),
            }
        )
    )

}

pub async fn read_questions() -> impl IntoResponse {
    (StatusCode::OK, 
        Json(
            [
            QuestionDetail {
                question_uuid: String::from("d347261c-3f0e-42d2-8706-5ef9f1b96725"),
                title: String::from("Newly Created Question"),
                description: String::from("My Description"),
                created_at: String::from("2024-09-11T12:00:00Z"),
            }
            ]
        )
    )
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
   StatusCode::OK
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    (StatusCode::OK, 
        Json(
            AnswerDetail {
                answer_uuid: String::from("43217890-a685-4695-9784-277246b16567"),
                question_uuid: answer.question_uuid.clone(),
                content: answer.content.clone(),
                created_at: String::from("2024-09-11T12:00:00Z"),
            }
        )
    )
}
// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
pub async fn read_answers(Json(question_id): Json<QuestionId>) -> impl IntoResponse {
    (StatusCode::OK, 
        Json(
            [
            AnswerDetail {
                answer_uuid: String::from("43217890-a685-4695-9784-277246b16567"),
                question_uuid: question_id.question_uuid.clone(),
                content: String::from("My Answer"),
                created_at: String::from("2024-09-11T12:00:00Z"),
            }
            ]
        )
    )
}

pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) -> impl IntoResponse {
    StatusCode::OK
}
// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
