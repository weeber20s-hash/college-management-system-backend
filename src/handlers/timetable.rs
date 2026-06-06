use axum::{Json, http::StatusCode, extract::{State, Path, Query}};
use uuid::Uuid;
use crate::config::AppState;
use crate::models::timetable::{Timetable, CreateTimetableRequest, UpdateTimetableRequest, GetTimetableQuery};

pub async fn create_timetable(State(state): State<AppState>, Json(payload): Json<CreateTimetableRequest>) -> Result<Json<Timetable>, (StatusCode, String)> {
    let item = sqlx::query_as::<_, Timetable>(
        "INSERT INTO timetables (course_code, day_of_week, start_time, end_time, room, lecturer, effective_date)
         VALUES ($1,$2,$3,$4,$5,$6,$7)
         RETURNING id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date"
    )
    .bind(&payload.course_code).bind(&payload.day_of_week).bind(payload.start_time).bind(payload.end_time)
    .bind(&payload.room).bind(&payload.lecturer).bind(payload.effective_date)
    .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(item))
}

pub async fn get_timetables(State(state): State<AppState>, Query(query): Query<GetTimetableQuery>) -> Result<Json<Vec<Timetable>>, (StatusCode, String)> {
    let items = if let Some(course_code) = query.course_code {
        sqlx::query_as::<_, Timetable>("SELECT id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date FROM timetables WHERE course_code = $1 ORDER BY day_of_week, start_time")
            .bind(course_code).fetch_all(&state.db_pool).await
    } else if let Some(day) = query.day_of_week {
        sqlx::query_as::<_, Timetable>("SELECT id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date FROM timetables WHERE LOWER(day_of_week)=LOWER($1) ORDER BY start_time")
            .bind(day).fetch_all(&state.db_pool).await
    } else if let Some(lecturer) = query.lecturer {
        sqlx::query_as::<_, Timetable>("SELECT id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date FROM timetables WHERE LOWER(lecturer) LIKE LOWER($1) ORDER BY day_of_week, start_time")
            .bind(format!("%{}%", lecturer)).fetch_all(&state.db_pool).await
    } else {
        sqlx::query_as::<_, Timetable>("SELECT id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date FROM timetables ORDER BY day_of_week, start_time")
            .fetch_all(&state.db_pool).await
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_timetable_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Timetable>, (StatusCode, String)> {
    let item = sqlx::query_as::<_, Timetable>("SELECT id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date FROM timetables WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    item.map(Json).ok_or((StatusCode::NOT_FOUND, "Timetable entry not found".to_string()))
}

pub async fn update_timetable(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<UpdateTimetableRequest>) -> Result<Json<Timetable>, (StatusCode, String)> {
    let current = sqlx::query_as::<_, Timetable>("SELECT id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date FROM timetables WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Timetable entry not found".to_string()))?;
    let updated = sqlx::query_as::<_, Timetable>(
        "UPDATE timetables SET course_code=$1, day_of_week=$2, start_time=$3, end_time=$4, room=$5, lecturer=$6, effective_date=$7 WHERE id=$8
         RETURNING id, course_code, day_of_week, start_time, end_time, room, lecturer, effective_date")
        .bind(payload.course_code.unwrap_or(current.course_code)).bind(payload.day_of_week.unwrap_or(current.day_of_week))
        .bind(payload.start_time.unwrap_or(current.start_time)).bind(payload.end_time.unwrap_or(current.end_time))
        .bind(payload.room.or(current.room)).bind(payload.lecturer.or(current.lecturer)).bind(payload.effective_date.or(current.effective_date)).bind(id)
        .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(updated))
}

pub async fn delete_timetable(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM timetables WHERE id=$1").bind(id).execute(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if result.rows_affected() == 0 { Err((StatusCode::NOT_FOUND, "Timetable entry not found".to_string())) } else { Ok(StatusCode::NO_CONTENT) }
}
