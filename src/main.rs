use axum::{routing::get, routing::post, routing::put, routing::delete, Router, middleware};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use axum_back::{config::{Config, AppState}, db::create_pool};
use axum_back::handlers::auth::{login, register};
use axum_back::handlers::admin_profile::{get_profile, update_profile, change_password};
use axum_back::middleware::auth::auth_middleware;
use axum_back::handlers::student::{
    create_student, get_students, get_student_by_id, update_student, delete_student
};
use axum_back::handlers::course::{
    create_course, get_courses, get_course_by_code, update_course, delete_course
};
use axum_back::handlers::attendance::{
    mark_attendance, get_attendance, get_attendance_summary, get_attendance_by_date
};
use axum_back::handlers::dashboard::{
    get_dashboard_stats, get_weekly_attendance, get_smart_alerts, get_upcoming_tasks
};
use axum_back::handlers::notice::{
    create_notice, get_all_notices, get_notice_by_id, update_notice, delete_notice
};
use axum_back::handlers::report::{
    student_report, attendance_report, course_report, date_range_report
};
use axum_back::handlers::timetable::{
    create_timetable, get_timetables, get_timetable_by_id, update_timetable, delete_timetable
};
use axum_back::handlers::assessment::{
    create_assessment, get_assessments, get_assessment_by_id, update_assessment, delete_assessment
};
use axum_back::handlers::grade::{
    create_grade, get_grades, get_grade_by_id, update_grade, delete_grade, get_student_gpa
};
use axum_back::handlers::fee::{
    create_fee, get_fees, get_fee_by_id, update_fee, delete_fee, get_fee_summary
};


#[tokio::main]
async fn main() {
    let config = Config::from_env();
    
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    let db_pool = match create_pool(&config.database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            return;
        }
    };
    
    match sqlx::migrate!().run(&db_pool).await {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => {
            eprintln!("Failed to run migrations: {}", e);
            return;
        }
    };
    
    let app_state = AppState {
        db_pool,
        jwt_secret: config.jwt_secret.clone(),
    };
    
    let profile_routes = Router::new()
        .route("/profile", get(get_profile))
        .route("/profile", put(update_profile))
        .route("/profile/change-password", post(change_password))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));
    
    let student_routes = Router::new()
        .route("/students", post(create_student))
        .route("/students", get(get_students))
        .route("/students/{id}", get(get_student_by_id))
        .route("/students/{id}", put(update_student))
        .route("/students/{id}", delete(delete_student))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let course_routes = Router::new()
        .route("/courses", post(create_course))
        .route("/courses", get(get_courses))
        .route("/courses/{course_code}", get(get_course_by_code))
        .route("/courses/{course_code}", put(update_course))
        .route("/courses/{course_code}", delete(delete_course))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let attendance_routes = Router::new()
        .route("/attendance", post(mark_attendance))
        .route("/attendance", get(get_attendance))
        .route("/attendance/summary/{student_id}/{course_code}", get(get_attendance_summary))
        .route("/attendance/date/{date}", get(get_attendance_by_date))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let dashboard_routes = Router::new()
        .route("/dashboard/stats", get(get_dashboard_stats))
        .route("/dashboard/weekly-attendance", get(get_weekly_attendance))
        .route("/dashboard/alerts", get(get_smart_alerts))
        .route("/dashboard/tasks", get(get_upcoming_tasks))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));
    let notice_routes = Router::new()
        .route("/notices", post(create_notice))
        .route("/notices", get(get_all_notices))
        .route("/notices/{id}", get(get_notice_by_id))
        .route("/notices/{id}", put(update_notice))
        .route("/notices/{id}", delete(delete_notice))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let report_routes = Router::new()
        .route("/reports/students", get(student_report))
        .route("/reports/attendance", get(attendance_report))
        .route("/reports/course", get(course_report))
        .route("/reports/date-range", get(date_range_report))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));



    let timetable_routes = Router::new()
        .route("/timetables", post(create_timetable))
        .route("/timetables", get(get_timetables))
        .route("/timetables/{id}", get(get_timetable_by_id))
        .route("/timetables/{id}", put(update_timetable))
        .route("/timetables/{id}", delete(delete_timetable))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let assessment_routes = Router::new()
        .route("/assessments", post(create_assessment))
        .route("/assessments", get(get_assessments))
        .route("/assessments/{id}", get(get_assessment_by_id))
        .route("/assessments/{id}", put(update_assessment))
        .route("/assessments/{id}", delete(delete_assessment))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let grade_routes = Router::new()
        .route("/grades", post(create_grade))
        .route("/grades", get(get_grades))
        .route("/grades/{id}", get(get_grade_by_id))
        .route("/grades/{id}", put(update_grade))
        .route("/grades/{id}", delete(delete_grade))
        .route("/grades/gpa/{student_id}", get(get_student_gpa))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let fee_routes = Router::new()
        .route("/fees", post(create_fee))
        .route("/fees", get(get_fees))
        .route("/fees/{id}", get(get_fee_by_id))
        .route("/fees/{id}", put(update_fee))
        .route("/fees/{id}", delete(delete_fee))
        .route("/fees/summary/{student_id}", get(get_fee_summary))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let public_routes = Router::new()
        .route("/", get(|| async { "Student Management System API" }))
        .route("/auth/login", post(login))
        .route("/auth/register", post(register));
    
    let app = Router::new()
        .merge(public_routes)
        .merge(profile_routes) 
        .merge(student_routes)
        .merge(course_routes)
        .merge(attendance_routes)
        .merge(dashboard_routes)
        .merge(notice_routes)
        .merge(report_routes)
        .merge(timetable_routes)
        .merge(assessment_routes)
        .merge(grade_routes)
        .merge(fee_routes)
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!("Server listening on {}", addr);
    
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return;
        }
    };
    
    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Server error: {}", e);
        }
    }
}