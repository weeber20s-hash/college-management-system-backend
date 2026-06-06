use axum::{Json, http::StatusCode, extract::{State, Query}};
use crate::config::AppState;
use crate::models::report::{
    StudentReport, AttendanceReport, CourseReport, DateRange,
    CourseStat, CourseAttendanceStat, LowAttendanceStudent
};

pub async fn student_report(
    State(state): State<AppState>,
) -> Result<Json<StudentReport>, (StatusCode, String)> {
    let status_stats = sqlx::query!(
        "SELECT status, COUNT(*) as count FROM students GROUP BY status"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get status stats error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let mut total = 0;
    let mut active = 0;
    let mut inactive = 0;
    let mut graduated = 0;
    
    for stat in status_stats {
        total += stat.count.unwrap_or(0);
        match stat.status.as_str() {
            "Active" => active = stat.count.unwrap_or(0),
            "Inactive" => inactive = stat.count.unwrap_or(0),
            "Graduated" => graduated = stat.count.unwrap_or(0),
            _ => {}
        }
    }
    
    let course_stats = sqlx::query!(
        "SELECT course, COUNT(*) as count FROM students GROUP BY course ORDER BY count DESC"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get course stats error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let students_by_course = course_stats
        .into_iter()
        .map(|stat| CourseStat {
            course_name: stat.course,
            student_count: stat.count.unwrap_or(0),
        })
        .collect();
    
    Ok(Json(StudentReport {
        total_students: total,
        active_students: active,
        inactive_students: inactive,
        graduated_students: graduated,
        students_by_course,
    }))
}

pub async fn attendance_report(
    State(state): State<AppState>,
) -> Result<Json<AttendanceReport>, (StatusCode, String)> {
    let overall = sqlx::query!(
        "SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN status = true THEN 1 ELSE 0 END) as present
         FROM attendance"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get overall attendance error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let total_classes = overall.total.unwrap_or(0);
    let total_present = overall.present.unwrap_or(0);
    let total_absent = total_classes - total_present;
    let overall_attendance_rate = if total_classes > 0 {
        (total_present as f64 / total_classes as f64) * 100.0
    } else {
        0.0
    };

    let course_attendance = sqlx::query!(
        "SELECT 
            c.course_code,
            c.course_name,
            COUNT(a.id) as total_classes,
            SUM(CASE WHEN a.status = true THEN 1 ELSE 0 END) as present_count
         FROM courses c
         LEFT JOIN attendance a ON c.course_code = a.course_code
         GROUP BY c.course_code, c.course_name
         ORDER BY c.course_code"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get course attendance error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let attendance_by_course = course_attendance
        .into_iter()
        .map(|stat| {
            let total = stat.total_classes.unwrap_or(0);
            let present = stat.present_count.unwrap_or(0);
            let rate = if total > 0 {
                (present as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            CourseAttendanceStat {
                course_code: stat.course_code,
                course_name: stat.course_name,
                attendance_rate: rate,
                total_classes: total,
            }
        })
        .collect();

    let low_attendance = sqlx::query!(
        "SELECT 
            s.student_id,
            s.full_name,
            s.course,
            COUNT(a.id) as total_classes,
            SUM(CASE WHEN a.status = true THEN 1 ELSE 0 END) as present_count
         FROM students s
         LEFT JOIN attendance a ON s.id = a.student_id
         GROUP BY s.student_id, s.full_name, s.course
         HAVING COUNT(a.id) > 0 
         AND (SUM(CASE WHEN a.status = true THEN 1 ELSE 0 END)::float / COUNT(a.id)) < 0.75
         ORDER BY (SUM(CASE WHEN a.status = true THEN 1 ELSE 0 END)::float / COUNT(a.id)) ASC"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get low attendance error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let low_attendance_students = low_attendance
        .into_iter()
        .map(|stat| {
            let total = stat.total_classes.unwrap_or(0);
            let present = stat.present_count.unwrap_or(0);
            let percentage = if total > 0 {
                (present as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            LowAttendanceStudent {
                student_id: stat.student_id,
                full_name: stat.full_name,
                course: stat.course,
                attendance_percentage: percentage,
            }
        })
        .collect();
    
    Ok(Json(AttendanceReport {
        overall_attendance_rate,
        total_classes,
        total_present,
        total_absent,
        attendance_by_course,
        low_attendance_students,
    }))
}

pub async fn course_report(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<CourseReport>, (StatusCode, String)> {
    let course_code = params.get("course_code")
        .ok_or((StatusCode::BAD_REQUEST, "Missing course_code parameter".to_string()))?;

    let course_data = sqlx::query!(
        "SELECT 
            c.course_code,
            c.course_name,
            c.lecturer,
            COUNT(DISTINCT s.id) as total_students
         FROM courses c
         LEFT JOIN students s ON c.course_code = s.course
         WHERE c.course_code = $1
         GROUP BY c.course_code, c.course_name, c.lecturer",
        course_code
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get course data error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let course_data = match course_data {
        Some(data) => data,
        None => return Err((StatusCode::NOT_FOUND, "Course not found".to_string())),
    };

    let attendance = sqlx::query!(
        "SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN status = true THEN 1 ELSE 0 END) as present
         FROM attendance
         WHERE course_code = $1",
        course_code
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get course attendance error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let total = attendance.total.unwrap_or(0);
    let present = attendance.present.unwrap_or(0);
    let attendance_rate = if total > 0 {
        (present as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(Json(CourseReport {
        course_code: course_data.course_code,
        course_name: course_data.course_name,
        lecturer: course_data.lecturer,
        total_students: course_data.total_students.unwrap_or(0),
        attendance_rate,
    }))
}
pub async fn date_range_report(
    State(state): State<AppState>,
    Query(range): Query<DateRange>,
) -> Result<Json<AttendanceReport>, (StatusCode, String)> {
    let stats = sqlx::query!(
        "SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN status = true THEN 1 ELSE 0 END) as present
         FROM attendance
         WHERE date BETWEEN $1 AND $2",
        range.start_date,
        range.end_date
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get date range stats error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let total_classes = stats.total.unwrap_or(0);
    let total_present = stats.present.unwrap_or(0);
    let total_absent = total_classes - total_present;
    let overall_attendance_rate = if total_classes > 0 {
        (total_present as f64 / total_classes as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(Json(AttendanceReport {
        overall_attendance_rate,
        total_classes,
        total_present,
        total_absent,
        attendance_by_course: vec![],
        low_attendance_students: vec![],
    }))
}