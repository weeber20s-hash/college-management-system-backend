use axum::{Json, http::StatusCode, extract::State};
use chrono::{Duration, Utc};
use crate::config::AppState;
use crate::models::dashboard::{DashboardStats, WeeklyAttendance, SmartAlert, UpcomingTask};


pub async fn get_dashboard_stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStats>, (StatusCode, String)> {
    let student_count = sqlx::query!("SELECT COUNT(*) as count FROM students")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Get student count error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .count
        .unwrap_or(0);

    let active_courses = sqlx::query!(
        "SELECT COUNT(DISTINCT course) as count FROM students WHERE status = 'Active'"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get active courses error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let active_courses_count = active_courses.count.unwrap_or(0);

    let attendance_stats = sqlx::query!(
        "SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN status = true THEN 1 ELSE 0 END) as present
         FROM attendance"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get attendance stats error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let total_attendance = attendance_stats.total.unwrap_or(0);
    let total_present = attendance_stats.present.unwrap_or(0);
    let average_attendance_rate = if total_attendance > 0 {
        (total_present as f64 / total_attendance as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(Json(DashboardStats {
        student_count,
        active_courses_count,
        average_attendance_rate,
        total_attendance_records: total_attendance,
    }))
}

pub async fn get_weekly_attendance(
    State(state): State<AppState>,
) -> Result<Json<Vec<WeeklyAttendance>>, (StatusCode, String)> {
    let mut weekly_data = Vec::new();
    let today = Utc::now().date_naive();
    
    for i in (0..4).rev() {
        let week_end = today - Duration::days(i * 7);
        let week_start = week_end - Duration::days(6);
        
        let stats = sqlx::query!(
            "SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN status = true THEN 1 ELSE 0 END) as present
             FROM attendance 
             WHERE date BETWEEN $1 AND $2",
            week_start,
            week_end
        )
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Get weekly attendance error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        
        let total = stats.total.unwrap_or(0);
        let present = stats.present.unwrap_or(0);
        let attendance_percentage = if total > 0 {
            (present as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        weekly_data.push(WeeklyAttendance {
            week_start,
            week_end,
            attendance_percentage,
        });
    }
    
    Ok(Json(weekly_data))
}

pub async fn get_smart_alerts(
    State(state): State<AppState>,
) -> Result<Json<Vec<SmartAlert>>, (StatusCode, String)> {
    let mut alerts = Vec::new();
    
    let low_attendance = sqlx::query!(
        "SELECT 
            s.id, s.full_name, s.course,
            COUNT(a.id) as total_classes,
            SUM(CASE WHEN a.status = true THEN 1 ELSE 0 END) as present_count
         FROM students s
         LEFT JOIN attendance a ON s.id = a.student_id
         GROUP BY s.id, s.full_name, s.course
         HAVING (SUM(CASE WHEN a.status = true THEN 1 ELSE 0 END)::float / NULLIF(COUNT(a.id), 0)) < 0.75
         AND COUNT(a.id) > 0"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get low attendance error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    if !low_attendance.is_empty() {
        let student_list: Vec<String> = low_attendance.iter()
            .take(3)
            .map(|s| s.full_name.clone())
            .collect();
        
        alerts.push(SmartAlert {
            alert_type: "warning".to_string(),
            title: "Low Attendance Alert".to_string(),
            message: format!("{} student(s) have attendance below 75%: {}", 
                low_attendance.len(),
                student_list.join(", ")
            ),
            priority: 4,
        });
    }
    
    let inactive_students = sqlx::query!(
        "SELECT COUNT(*) as count FROM students WHERE status = 'Inactive'"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get inactive students error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let inactive_count = inactive_students.count.unwrap_or(0);
    if inactive_count > 10 {
        alerts.push(SmartAlert {
            alert_type: "info".to_string(),
            title: "Inactive Students".to_string(),
            message: format!("{} students are currently inactive. Consider reaching out to them.", inactive_count),
            priority: 2,
        });
    }
    
    let total_students = sqlx::query!("SELECT COUNT(*) as count FROM students")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Get total students error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    let student_count = total_students.count.unwrap_or(0);
    alerts.push(SmartAlert {
        alert_type: "success".to_string(),
        title: "Total Enrollment".to_string(),
        message: format!("Currently managing {} student(s) in the system", student_count),
        priority: 1,
    });

    let empty_courses = sqlx::query!(
        "SELECT COUNT(*) as count FROM courses c 
         LEFT JOIN students s ON c.course_code = s.course 
         WHERE s.id IS NULL"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get empty courses error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let empty_courses_count = empty_courses.count.unwrap_or(0);
    if empty_courses_count > 0 {
        alerts.push(SmartAlert {
            alert_type: "info".to_string(),
            title: "Courses with No Students".to_string(),
            message: format!("{} courses have no enrolled students", empty_courses_count),
            priority: 2,
        });
    }
    
    Ok(Json(alerts))
}

pub async fn get_upcoming_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<UpcomingTask>>, (StatusCode, String)> {
    let mut tasks = Vec::new();
    let today = Utc::now().date_naive();

    let attendance_today = sqlx::query!(
        "SELECT COUNT(*) as count FROM attendance WHERE date = $1",
        today
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get today's attendance error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let total_students = sqlx::query!("SELECT COUNT(*) as count FROM students WHERE status = 'Active'")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Get total students error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    let marked_today = attendance_today.count.unwrap_or(0);
    let total_active = total_students.count.unwrap_or(0);
    
    if marked_today < total_active {
        tasks.push(UpcomingTask {
            header: "Mark Today's Attendance".to_string(),
            content: format!("{}/{} students have been marked present today. Please complete attendance marking.", marked_today, total_active),
            due_date: Some(today),
        });
    }
    
    let inactive_count = sqlx::query!("SELECT COUNT(*) as count FROM students WHERE status = 'Inactive'")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Get inactive count error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    let inactive = inactive_count.count.unwrap_or(0);
    if inactive > 0 {
        tasks.push(UpcomingTask {
            header: "Review Inactive Students".to_string(),
            content: format!("{} students are marked as inactive. Review their status and take necessary action.", inactive),
            due_date: None,
        });
    }
    
    tasks.push(UpcomingTask {
        header: "Weekly Attendance Review".to_string(),
        content: "Review this week's attendance analytics and identify students with low attendance rates.".to_string(),
        due_date: Some(today + Duration::days(7)),
    });
    
    Ok(Json(tasks))
}